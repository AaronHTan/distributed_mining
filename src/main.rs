use tokio;

mod server;
use server::{ServerClosed, ServerCreated, ServerRunning};
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Hello, world!");
    let s = ServerCreated::builder().run().unwrap();
    s.write(server::MessageBus {}).unwrap();
}
