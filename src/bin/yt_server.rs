extern crate zmq;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate youtube_downloader;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::Drain;
use std::path::PathBuf;
use structopt::StructOpt;
use youtube_downloader::{job::Job, server::Server};

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: PathBuf,
    #[structopt(short = "s", long = "sleep-time", default_value = "86400")]
    sleep_time: u64,
}

fn main() {
    let opts = Opts::from_args();
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!("program" => "yt-server"));

    let server = Server::new(opts.config, opts.sleep_time, log);
    server.run().unwrap();
}
