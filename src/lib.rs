#[cfg(test)]
#[macro_use]
extern crate hamcrest;
#[macro_use]
extern crate log;

mod backend;
pub mod commands;
pub mod frontend;
mod intermediate;

pub fn error(msg: &str) {
    println!("ERROR: {}", msg);
}

#[cfg(test)]
mod tests {}
