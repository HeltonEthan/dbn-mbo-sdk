use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct Config {
    pub path: PathBuf,
}

impl Config {
    pub fn path(&mut self, path: &PathBuf) {
        self.path = PathBuf::from(path);
    }
}
