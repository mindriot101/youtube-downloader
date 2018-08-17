#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate toml;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

pub mod config;
mod download_thread;
pub mod job;
pub mod server;
