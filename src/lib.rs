use std::fs;
use std::path::Path;

pub fn new(name: &str) {
    fs::create_dir(name);
}

pub fn init(path: &str) {
    println!("initalizing at {}", path);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
