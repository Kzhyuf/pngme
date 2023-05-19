extern crate core;
// pub mod args;
pub mod chunk;
pub mod chunk_type;
// pub mod commands;
pub mod png;
pub mod clap;

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;