/// Distibuted Server for handling Mining Operations. Handles new connections, drops from various
/// connections, as well as ways to distribute new tasks via (write) as well as get completed tasks
/// via (read). You can also specify specific clients to send data to.
/// There is also an init method. Connection errors are handled and given as errors either to the
/// specified readDrops or if used with a specific connection, an error is given there. There is
/// a specific handler for each client function.
#[allow(unused_imports)]
use std::{sync::mpsc, thread};
use tokio::{self, sync::mpsc as tmpsc};

use tokio_stream::{self as stream, StreamExt};

struct Message;
#[allow(dead_code)]

struct ClientConn {
    receiver: mpsc::Receiver<Message>,
}

pub struct Server {
    read_channel: 
}

#[allow(dead_code)]
impl Server {
    pub fn new() -> Server {
        let (work_tx, mut work_rx) = tmpsc::channel::<()>(1);
        Server {}
    }

    /// serve is the main loop that handles various events atomically, preventing any dataraces.
    /// It interacts with numerous other threads, including ones that store various client
    /// interaction data, the main listening loop for handling new connections, and also
    /// interacting with client functions as well.
    ///
    async fn serve(& self) {
        let mut stream1 = stream::iter(vec![1, 2, 3]);
        let mut stream2 = stream::iter(vec![4, 5, 6]);
        let mut values = vec![];
        loop {
            tokio::select! {
                Some(v) = stream1.next() => values.push(v),
                Some(v) = stream2.next() => values.push(v),
                else => break,
            }
        }
    }
    pub fn read() {}

    pub fn write() {}

    pub fn read_id() {}

    pub fn write_id() {}

    pub fn is_valid() {}

    pub fn client_list() {}

    pub fn close() {}
}

fn read_to_client(client_id: u32) {

}

fn write_to_client() {

}
