#![feature(try_from)]

///! The main module of Mini VM.

#[cfg(test)]
#[macro_use]
extern crate hamcrest;
#[macro_use]
extern crate log;
extern crate byteorder;

pub mod backend;
pub mod commands;
pub mod frontend;
pub mod intermediate;

/// Prints custom error message to STDERR.
pub fn error(msg: &str) {
    eprintln!("ERROR: {}", msg);
}

#[cfg(test)]
mod tests {}
