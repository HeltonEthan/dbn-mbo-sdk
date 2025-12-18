use chrono::NaiveDate;
use color_eyre::eyre::Result;
use std::path::PathBuf;

use crate::helper;

/// Config gives the engine specific details on where and what to run.
/// 
/// This struct holds information about the directory the engine will look at
/// and what files it should consider in regards to start and end.
#[derive(Debug)]
pub struct Config {
    pub dir: PathBuf,
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl Config {
    /// Creates a new config with given dir, start, and end
    pub fn new(dir: PathBuf, start: NaiveDate, end: NaiveDate) -> Self {
        Self { dir, start, end }
    }

    /// Returns a reference to the dir.
    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }

    /// Returns a reference to start.
    pub fn start(&self) -> &NaiveDate {
        &self.start
    }

    /// Returns a reference to end.
    pub fn end(&self) -> &NaiveDate {
        &self.end
    }

    /// Returns a start as unix_nanos.
    pub fn start_unix(&self) -> Result<u64> {
        helper::to_unix(self.start())
    }

    /// Returns a end as unix_nanos.
    pub fn end_unix(&self) -> Result<u64> {
        helper::to_unix(self.end())
    }
}
