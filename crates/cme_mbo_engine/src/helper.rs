use chrono::{NaiveDate, NaiveTime};
use color_eyre::eyre::Result;
use std::path::PathBuf;

pub fn to_unix(date: &NaiveDate) -> Result<u64> {
    let dt_unix = date
        .and_time(NaiveTime::from_hms_nano_opt(0, 0, 0, 0).unwrap())
        .and_utc()
        .timestamp() as u64;
    Ok(dt_unix * 1_000_000_000)
}

#[allow(dead_code)]
pub fn str_to_pathbuf(str: String) -> Result<PathBuf> {
    Ok(PathBuf::from(str))
}

#[allow(dead_code)]
pub fn str_to_naivedate(str: String) -> Result<NaiveDate> {
    Ok(NaiveDate::parse_from_str(&str, "%Y-%m-%d")?)
}
