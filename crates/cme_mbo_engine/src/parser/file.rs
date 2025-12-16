use color_eyre::eyre::Result;
use std::ffi::OsStr;
use std::{fs, num::NonZero, path::PathBuf};
use crate::{
    config::Config
};
use crate::parser::dbn;

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

#[cfg(test)]
mod test {
    use super::*;

    use crate::helper;

    #[test]
    pub fn get_files() -> Result<()> {
        println!("-----get_files-----");
        let dir = helper::str_to_pathbuf("C:/Users/helto/GLBX-20250915-NGKNUL4VBG".to_string())?;
        let start = helper::str_to_naivedate("2025-05-14".to_string()).unwrap();
        let end = helper::str_to_naivedate("2025-05-23".to_string()).unwrap();
        println!("dir: {:#?}", &dir);
        println!("start: {}", &start);
        println!("end: {}", &end);
        let mut files_in_dir = PathBuf::new();
        for file in fs::read_dir(&dir)? {
            let file = file?;
            let path = file.path();
            println!("path extension: {:#?}", path.extension());
            if !path.is_file() || path.extension() != Some(OsStr::new("zst")) {
                continue;
            }
            let file_metadata = dbn::decode_metadata(&path)?;
            println!("file_metadata.start: {:#?}", file_metadata.start);
            println!("file_metadata.end: {:#?}", file_metadata.end);
            if helper::to_unix(&start)? <= file_metadata.start && file_metadata.start <= helper::to_unix(&end)?
                || Some(NonZero::new(helper::to_unix(&start)?).unwrap()) <= file_metadata.end
                    && file_metadata.end <= Some(NonZero::new(helper::to_unix(&end)?).unwrap())
            {
                files_in_dir.push(path);
            }
        }
        println!("files in directory: {:#?}", files_in_dir);
        println!();
        Ok(())
    }
}
