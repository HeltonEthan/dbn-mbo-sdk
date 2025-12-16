use color_eyre::eyre::Result;
use dbn::{
    Action,
    decode::{
        DecodeStream,
        dbn::{Decoder, MetadataDecoder},
    },
    record::MboMsg,
};
use fallible_streaming_iterator::FallibleStreamingIterator;
use std::{fs::File, io::BufReader, path::PathBuf};
use crate::{
    config::Config
};
use crate::orderbook::market::Market;
use crate::parser::file;

#[inline]
#[allow(dead_code)]
pub fn run<F: FnMut(&MboMsg) -> Option<Action>>(mut logic: F, cfg: &Config) -> Result<()> {
    let start_unix = cfg.start_unix()?;
    let end_unix = cfg.end_unix()?;
    let mut market = Market::default();
    for path in file::get_files(&cfg)?.iter() {
        let mut dbn_stream = Decoder::from_zstd_file(path)?.decode_stream::<MboMsg>();
        let _symbol_map = decode_metadata(path)?.symbol_map()?;
        while let Some(mbo_msg) = dbn_stream.next()? {
            if mbo_msg.ts_recv < start_unix {
                continue;
            }
            if mbo_msg.ts_recv > end_unix {
                break;
            }
            market.apply(mbo_msg.clone());
            logic(&mbo_msg);
        }
    }
    Ok(())
}

pub fn decode_metadata(path: &PathBuf) -> Result<dbn::Metadata> {
    let reader = zstd::stream::Decoder::new(BufReader::new(File::open(path)?)).unwrap();
    Ok(MetadataDecoder::new(reader).decode()?)
}

#[cfg(test)]
mod test {
    use super::*;
    use ::dbn::SymbolIndex;

    use crate::helper;
    use crate::parser::dbn;

    #[test]
    pub fn dbn_stream() -> Result<()> {
        let path = PathBuf::from(r"C:/Users/helto/GLBX-20250915-NGKNUL4VBG/glbx-mdp3-20250512-20250517.mbo.dbn.zst");
        let start_unix = None;
        let end_unix = None;
        let mut dbn_stream = Decoder::from_zstd_file(path)?.decode_stream::<MboMsg>();
        while let Ok(Some(mbo_msg)) = dbn_stream.next() {
            if let Some(start_unix) = start_unix {
                if mbo_msg.ts_recv < start_unix {
                    continue;
                }
            }
            if let Some(end_unix) = end_unix {
                if mbo_msg.ts_recv > end_unix {
                    break;
                }
            }
            _ = mbo_msg;
        }
        Ok(())
    }

    #[test]
    pub fn run() -> Result<()> {
        let cfg = Config::new(
            helper::str_to_pathbuf("C:/Users/helto/GLBX-20250915-NGKNUL4VBG".to_string())?,
            helper::str_to_naivedate("2025-05-12".to_string()).unwrap(),
            helper::str_to_naivedate("2025-05-17".to_string()).unwrap(),
        );
        let start_unix = cfg.start_unix()?;
        let end_unix = cfg.end_unix()?;
        let mut market = Market::default();
        for path in file::get_files(&cfg)?.iter() {
            let mut dbn_stream = Decoder::from_zstd_file(path)?.decode_stream::<MboMsg>();
            let symbol_map = dbn::decode_metadata(path)?.symbol_map()?;
            while let Some(mbo_msg) = dbn_stream.next()? {
                if mbo_msg.ts_recv < start_unix {
                    continue;
                }
                if mbo_msg.ts_recv > end_unix {
                    let symbol = symbol_map.get_for_rec(mbo_msg).unwrap();
                    let (best_bid, best_offer) = market.aggregated_bbo(mbo_msg.hd.instrument_id);
                    println!("{symbol} Aggregated BBO | {}", mbo_msg.ts_recv().unwrap());
                    if let Some(best_offer) = best_offer {
                        println!("    {best_offer}");
                    } else {
                        println!("    None");
                    }
                    if let Some(best_bid) = best_bid {
                        println!("    {best_bid}");
                    } else {
                        println!("    None");
                    }
                    break;
                }
                market.apply(mbo_msg.clone());
            }
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
