#[cfg(test)]
#[macro_use]
extern crate hamcrest;

pub mod frontend;
pub mod commands;

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
