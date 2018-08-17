extern crate zmq;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate youtube_downloader;

use structopt::StructOpt;
use std::error::Error;
use std::path::PathBuf;
use youtube_downloader::job::Job;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: PathBuf,
    #[structopt(short = "s", long = "sleep-time", default_value = "86400")]
    sleep_time: u32,
}


fn main() {
    let opts = Opts::from_args();
}
