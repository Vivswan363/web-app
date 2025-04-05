#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};
use tokio;

fn main() {
    let mut server = Nickel::new();
    server.get("**", middleware!(middleware()));
    let _ = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(server.listen("127.0.0.1:6767"));
}

fn middleware() -> &'static str {
    "Hello World!" 
}