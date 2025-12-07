use std::path::PathBuf;
use color_eyre::eyre::Result;
use dbn::{
    decode::{DecodeStream, dbn::Decoder},
    record::MboMsg,
};
use fallible_streaming_iterator::FallibleStreamingIterator;

pub fn dbn_stream(path: &PathBuf) -> Result<()> {
    let mut dbn_stream = Decoder::from_zstd_file(path)?.decode_stream::<MboMsg>();
    while let Ok(Some(mbo_msg)) = dbn_stream.next() {
        println!("{mbo_msg:?}");
    }

    Ok(())
}
