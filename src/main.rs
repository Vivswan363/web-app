extern crate chrono;

use std::io::prelude::*;
use std::fs::File;
use std::io;
use chrono::*;
use chrono_tz::Asia::Kolkata;
use chrono_tz::Tz;

fn log_time(filename: &str, timezone: &Tz) -> io::Result<()> {
    let local = Utc::now().with_timezone(timezone);
    let mut time_str = local.format("%c").to_string();
    time_str.push_str("\n");
    let bytes = time_str.as_bytes();
    let mut f = File::create(filename)?;
    f.write_all(bytes)?;
    Ok(())
}

fn main() {
    match log_time("log.txt", &Kolkata) {
        Ok(..) => println!("File created!"),
        Err(..) => println!("Error: could not create file.")
    }
}