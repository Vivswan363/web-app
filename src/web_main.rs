use nickel::*;
use tokio;

pub fn write_on_server(string_to_write: &'static str) {
    let mut server = Nickel::new();
    server.get("**", middleware!(|_, response1| {
        let response = String::from(string_to_write);
        response
    }));
    let _ = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(server.listen("127.0.0.1:6767"));
}
