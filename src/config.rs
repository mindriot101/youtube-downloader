use failure::Error;
use job::Job;
use std::fs;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) jobs: Vec<Job>,
}

impl Config {
    pub(crate) fn from_file<P>(filename: P) -> Result<Config, Error>
    where
        P: AsRef<Path>,
    {
        let text = fs::read_to_string(filename)?;
        let deserialized: Config = toml::from_str(&text)?;
        Ok(deserialized)
    }
}
