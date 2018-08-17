extern crate zmq;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate youtube_downloader;

use std::path::PathBuf;
use structopt::StructOpt;
use youtube_downloader::{job::Job, server::Server};

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: PathBuf,
    #[structopt(short = "s", long = "sleep-time", default_value = "86400")]
    sleep_time: u32,
}

fn main() {
    let opts = Opts::from_args();
    let server = Server::new(opts.config, opts.sleep_time);
    server.run().unwrap();
}
