use crate::config::Config;
use crate::parser::dbn;
use color_eyre::eyre::Result;
use std::ffi::OsStr;
use std::{fs, num::NonZero, path::PathBuf};

/// Loops through all files in the directory and creates a Vec<PathBuf>
/// of paths that you want to run<logic>() on.
pub fn get_files(config: &Config) -> Result<Vec<PathBuf>> {
    let mut files_in_dir = Vec::new();
    for file in fs::read_dir(config.dir())? {
        let file = file?;
        let path = file.path();
        if !path.is_file() || path.extension() != Some(OsStr::new("zst")) {
            continue;
        }
        let file_metadata = dbn::decode_metadata(&path)?;
        if config.start_unix()? <= file_metadata.start && file_metadata.start <= config.end_unix()?
            || Some(NonZero::new(config.start_unix()?).unwrap()) <= file_metadata.end
                && file_metadata.end <= Some(NonZero::new(config.end_unix()?).unwrap())
        {
            files_in_dir.push(path)
        }
    }
    Ok(files_in_dir)
}
