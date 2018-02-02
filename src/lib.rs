#[cfg(test)]
#[macro_use]
extern crate hamcrest;

mod backend;
pub mod commands;
pub mod frontend;
mod model;

pub fn error(msg: &str) {
    println!("ERROR: {}", msg);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
