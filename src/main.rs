#![feature(tuple_trait)]
#![feature(unboxed_closures)]
use tokio;
mod client;
mod message;
mod server;

use client::ClientBuild;
use server::ServerCreated;
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Hello, world!");
    let mut s = ServerCreated::builder().run().await.unwrap();
    let mut c = ClientBuild::build()
        .add_server("127.0.0.1:8080")
        .connect()
        .await
        .unwrap();

    c.write("OK".as_bytes()).await.unwrap();
    s.write("WOW".as_bytes()).await.unwrap();
}
