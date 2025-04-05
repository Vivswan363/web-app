use chrono::*;
use chrono_tz::Asia::Kolkata;
use chrono_tz::Tz;
use std::io::prelude::*;
use std::fs::File;
use std::io;

fn _log_something(filename: &str, string: &[u8]) -> Result<(), io::Error> {
    let mut f = (File::create(filename))?;
    f.write_all(string)
}

fn write_file(filename: &str, timezone: &Tz) -> Result<(), io::Error> {
    let time = Utc::now().with_timezone(timezone);
    // time.format(fmt)
    let time_string = format!("current time({}): {}:{}:{}\n", time.offset(), time.hour(), time.minute(), time.second());
    let mut file = File::create(filename)?;
    file.write_all(time_string.as_bytes())?;
    Ok(())
}


fn main() {
    write_file("date-time.txt", &Kolkata).expect("Error: unable to write file");
}
