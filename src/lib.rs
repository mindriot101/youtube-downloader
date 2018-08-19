#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate toml;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate zmq;

pub mod config;
mod download_thread;
mod handler_thread;
pub mod job;
pub mod server;
