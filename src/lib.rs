#[cfg(test)]
#[macro_use]
extern crate hamcrest;
#[macro_use]
extern crate log;

mod backend;
pub mod commands;
pub mod frontend;
mod intermediate;

/// Prints custom error message to STDERR.
pub fn error(msg: &str) {
    eprintln!("ERROR: {}", msg);
}

#[cfg(test)]
mod tests {}
