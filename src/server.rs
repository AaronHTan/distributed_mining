/// Distibuted Server for handling Mining Operations. Handles new connections, drops from various
/// connections, as well as ways to distribute new tasks via (write) as well as get completed tasks
/// via (read). You can also specify specific clients to send data to.
/// There is also an init method. Connection errors are handled and given as errors either to the
/// specified readDrops or if used with a specific connection, an error is given there. There is
/// a specific handler for each client function.
#[allow(unused_imports)]
use std::sync::mpsc;

struct Message;
#[allow(dead_code)]
struct ClientConn {
    
    receiver: mpsc::Receiver<Message>,

}

struct Server {}

#[allow(dead_code)]
impl Server {
    fn new() {}

    fn read() {}

    fn write() {}

    fn read_id() {}

    fn write_id() {}
}
