use anyhow::{bail, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::Rss;

pub fn read_file(path: &Path) -> Result<Rss> {
    match File::open(path) {
        Ok(f) => Ok(quick_xml::de::from_reader(BufReader::new(f))?),
        Err(_) => bail!("Cannot read file in path '{}'", path.display()),
    }
}
