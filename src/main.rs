use tokio;

mod server;
use server::Server;
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Hello, world!");
    let s = Server::new();
}
