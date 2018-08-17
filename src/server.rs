use failure::Error;
use std::path::PathBuf;

pub struct Server {
    config: PathBuf,
    sleep_time: u32,
}

impl Server {
    pub fn new(config: PathBuf, sleep_time: u32) -> Self {
        Server { config, sleep_time }
    }

    pub fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}
