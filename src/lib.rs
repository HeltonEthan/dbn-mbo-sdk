mod config;
pub use config::Config;

//parser
mod parser;
pub use parser::run::{

};
#[cfg(test)]
pub mod test {
    use std::path::PathBuf;
    use color_eyre::eyre::Result;
    use dbn::{
        decode::{DecodeStream, dbn::Decoder},
        record::MboMsg,
    };
    use fallible_streaming_iterator::FallibleStreamingIterator;
    use crate::Config;

    #[test]
    pub fn dbn_stream() -> Result<()> {
        let path = PathBuf::from(r"C:/Users/helto/GLBX-20250915-NGKNUL4VBG/glbx-mdp3-20250512-20250517.mbo.dbn.zst");
        let mut config: Config = Default::default();
        config.path(&path);

        let mut dbn_stream = Decoder::from_zstd_file(&config.path)?.decode_stream::<MboMsg>();
        while let Ok(Some(mbo_msg)) = dbn_stream.next() {
            continue
        }

        Ok(())
    }
}
