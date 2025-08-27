use tokio;
mod client;
mod message;
mod server;

use client::ClientBuild;
use server::{ServerClosed, ServerCreated, ServerRunning};
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Hello, world!");
    let s = ServerCreated::builder().run().unwrap();
    let c = ClientBuild::build()
        .add_server("8080")
        .connect()
        .await
        .unwrap();

    c.write("OK".as_bytes);
    s.write("WOW".as_bytes());
}
