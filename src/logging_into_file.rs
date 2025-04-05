extern crate chrono;

use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs::File;
use std::io;
use chrono::*;
use chrono_tz::Asia::Kolkata;
use chrono_tz::Tz;

fn log_time_entry(timezone: &Tz) -> String {
    let local = Utc::now().with_timezone(timezone);
    let mut time_str = local.format("%c").to_string();
    time_str.push_str("\n");
    time_str 
    
}

fn log_time(filename: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().append(true).create(true).open(filename)?;
    let text_to_write = log_time_entry(&Kolkata);
    let text_as_bytes = text_to_write.as_bytes();
    file.write_all(text_as_bytes)?;
    Ok(())
}

fn main() {
    match log_time("log-time.txt") {
        Ok(..) => println!("File created!"),
        Err(..) => println!("Error: could not create file.")
    }
}