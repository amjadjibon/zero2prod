//! main.rs

use zero2prod::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .expect("failed to bind port");
    run(listener)?.await
}
