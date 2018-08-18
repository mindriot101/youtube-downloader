extern crate zmq;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate failure;
extern crate serde_json;
extern crate youtube_downloader;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use failure::Error;
use slog::{Drain, Logger};
use structopt::StructOpt;
use youtube_downloader::job::Job;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short = "u", long = "url")]
    url: String,
    #[structopt(short = "d", long = "dest")]
    dest: String,
    #[structopt(short = "a", long = "address", default_value = "localhost")]
    address: String,
    #[structopt(short = "p", long = "port", default_value = "5505")]
    port: u64,
}

fn add_job(job: Job, log: Logger, addr: &str) -> Result<(), Error> {
    info!(log, "adding job"; "job" => format!("{:?}", job));

    debug!(log, "deserialising input");
    let message = serde_json::to_vec(&job)?;
    let context = zmq::Context::new();
    let socket = context.socket(zmq::SocketType::REQ)?;

    debug!(log, "connecting to ZMQ socket"; "addr" => addr);
    socket.connect(addr)?;

    debug!(log, "sending message");
    socket.send(&message, 0)?;

    match socket.recv_string(0) {
        Ok(Ok(msg)) => assert_eq!(msg, "ok".to_string()),
        e => bail!("error: {:?}", e),
    }

    Ok(())
}

fn main() {
    let opts = Opts::from_args();

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!("program" => "yt-add"));

    let listen_addr = format!("tcp://{}:{}", opts.address, opts.port);
    let job = Job::new(opts.url, opts.dest);
    add_job(job, log, &listen_addr).unwrap();
}
