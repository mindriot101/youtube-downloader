extern crate zmq;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate serde_json;
extern crate youtube_downloader;

use failure::Error;
use structopt::StructOpt;
use youtube_downloader::job::Job;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short = "u", long = "url")]
    url: String,
    #[structopt(short = "d", long = "dest")]
    dest: String,
}

fn add_job(job: Job) -> Result<(), Error> {
    let message = serde_json::to_vec(&job)?;
    let context = zmq::Context::new();
    let socket = context.socket(zmq::SocketType::REQ)?;
    socket.connect("tcp://127.0.0.1:5505")?;
    socket.send(&message, 0)?;

    Ok(())
}

fn main() {
    let opts = Opts::from_args();
    let job = Job::new(opts.url, opts.dest);
    add_job(job).unwrap();
}
