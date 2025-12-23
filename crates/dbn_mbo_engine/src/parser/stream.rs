use dbn::{
    decode::{
        DecodeStream,
        dbn::{Decoder, MetadataDecoder},
    },
    record::MboMsg,
};
use fallible_streaming_iterator::FallibleStreamingIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{fs::File, io::BufReader, path::PathBuf};

use crate::api::{
    action::{Request},
    latency::LatencyModel,
};
use crate::orderbook::market::Market;
use crate::parser::file;
use crate::prelude::*;

/// Run is the entry point of the engine
///
/// It threads each file then iterate and streams through each,
/// it passes a clone of mbo_msg to the limit orderbook for reconstruction.
/// Then passes a reference of mbo to the callback function 'logic' and a 'LatencyModel'.
pub fn run<L, LF, LM, LMF>(cfg: &Config, logic_factory: LF, latency_model_factory: LMF) -> anyhow::Result<()>
where
    L: FnMut(&MboMsg) -> Option<Request>,
    LF: Fn() -> L + Sync,
    LM: LatencyModel,
    LMF: Fn() -> LM + Sync,
{
    let start_unix = cfg.start_unix()?;
    let end_unix = cfg.end_unix()?;
    let paths = file::get_files(&cfg)?;
    paths.par_iter().try_for_each(|path| -> anyhow::Result<()> {
        let mut dbn_stream = Decoder::from_zstd_file(path)?.decode_stream::<MboMsg>();
        let mut logic = logic_factory();
        let mut latency_model = latency_model_factory();
        let mut market = Market::new();
        while let Some(mbo_msg) = dbn_stream.next()? {
            if mbo_msg.ts_recv < start_unix {
                continue;
            }
            if mbo_msg.ts_recv > end_unix {
                break;
            }
            market.apply(mbo_msg.clone());
            if let Some(request) = logic(mbo_msg) {
                let _ = request;
            }
        }
        Ok(())
    })?;
    Ok(())
}

/// Returns the metadata of a path.
pub fn decode_metadata(path: &PathBuf) -> anyhow::Result<dbn::Metadata> {
    let reader = zstd::stream::Decoder::new(BufReader::new(File::open(path)?)).unwrap();
    Ok(MetadataDecoder::new(reader).decode()?)
}
