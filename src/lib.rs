#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate toml;

pub mod config;
mod download_thread;
pub mod job;
pub mod server;
