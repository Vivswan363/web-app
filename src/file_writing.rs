use std::io::prelude::*;
use std::fs::File;
use std::io;

fn log_something(filename: &'static str, string: &'static [u8]) -> Result<(), io::Error> {
    let mut f = (File::create(filename))?;
    f.write_all(string)
}

fn main() {
    match log_something("log.txt", b"ITS ALIVE!!!\n") {
        Ok(_) => println!("File created!"),
        Err(_) => println!("Error: could not create file.")
    }
}