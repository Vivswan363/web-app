use nickel::*;
use tokio;
extern crate chrono;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io;
use chrono::*;
use chrono_tz::Asia::Kolkata;


fn formatted_time_entry() -> String {
    let local = Utc::now().with_timezone(&Kolkata);
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn _record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().
                        append(true).
                        create(true).
                        open(format!("../{}", filename))?;
    file.write_all(bytes)?;
    Ok(())
}

fn _log_time(filename: &'static str) -> io::Result<()> {
    let entry = formatted_time_entry();
    let bytes = entry.as_bytes();

    _record_entry_in_log(filename, &bytes)?;
    Ok(())
}

fn write_on_server(text: String ) {
    let mut server = Nickel::new();
    server.get("**", middleware!(|_req, res| text.clone()));
    let _ = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(server.listen("127.0.0.1:6767"));
}


fn main() {
    write_on_server(formatted_time_entry());
    // log_time("log-time.txt").expect("Error: could not log time.");
}