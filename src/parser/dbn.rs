use std::{fs::File, path::PathBuf, io::BufReader};
use color_eyre::eyre::{Result};
use dbn::{
    decode::{
        DecodeStream,
        dbn::{Decoder, MetadataDecoder}
    },
    record::MboMsg,
    Action,
};
use fallible_streaming_iterator::FallibleStreamingIterator;

use crate::Config;
use crate::parser::file;

pub fn run<F: FnMut(&MboMsg) -> Option<Action>>(mut logic: F, cfg: &Config) -> Result<()> {
    let start_unix = cfg.start_unix()?;
    let end_unix = cfg.end_unix()?;
    
    for path in file::get_files(&cfg)?.iter(){
        let mut dbn_stream = Decoder::from_zstd_file(path)?.decode_stream::<MboMsg>();

        while let Some(mbo_msg) = dbn_stream.next()? {
            if mbo_msg.ts_recv < start_unix { continue; }
            if mbo_msg.ts_recv > end_unix { break; }

            logic(&mbo_msg);
        }
    }

    Ok(())
}

pub fn decode_metadata(path: &PathBuf) -> Result<dbn::Metadata> {
    let reader = zstd::stream::Decoder::new(BufReader::new(File::open(path)?)).unwrap();
    Ok(MetadataDecoder::new(reader).decode()?)
}

//test
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn dbn_stream() -> Result<()> {
        let path = PathBuf::from(
            r"C:/Users/helto/GLBX-20250915-NGKNUL4VBG/glbx-mdp3-20250512-20250517.mbo.dbn.zst"
        );
        let start_unix = None;
        let end_unix = None;

        let mut dbn_stream = Decoder::from_zstd_file(path)?.decode_stream::<MboMsg>();
        
        while let Ok(Some(mbo_msg)) = dbn_stream.next() {
            if let Some(start_unix) = start_unix { if mbo_msg.ts_recv < start_unix { continue; } }
            if let Some(end_unix) = end_unix { if mbo_msg.ts_recv > end_unix { break; } }

            _ = mbo_msg;
        }

        Ok(())
    }

    #[test]
    pub fn decode_metadata() -> Result<()> {
        let path = PathBuf::from(r"C:/Users/helto/GLBX-20250915-NGKNUL4VBG/glbx-mdp3-20250512-20250517.mbo.dbn.zst");
        let reader = zstd::stream::Decoder::new(BufReader::new(File::open(path)?)).unwrap();

        let _decode = MetadataDecoder::new(reader).decode()?;
        
        Ok(())
    }
}
