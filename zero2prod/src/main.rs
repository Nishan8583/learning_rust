use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // :0, means it lets os chose a random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind to any port");
    run(listener)?.await
}
