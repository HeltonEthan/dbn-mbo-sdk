use chrono::NaiveDate;
use color_eyre::eyre::Result;
use std::path::PathBuf;

use crate::helper;

#[derive(Debug)]
pub struct Config {
    pub dir: PathBuf,
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl Config {
    pub fn new(dir: PathBuf, start: NaiveDate, end: NaiveDate) -> Self {
        Self { dir, start, end }
    }

    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }

    pub fn start(&self) -> &NaiveDate {
        &self.start
    }

    pub fn end(&self) -> &NaiveDate {
        &self.end
    }

    pub fn start_unix(&self) -> Result<u64> {
        helper::to_unix(self.start())
    }

    pub fn end_unix(&self) -> Result<u64> {
        helper::to_unix(self.end())
    }
}
