/// Distibuted Server for handling Mining Operations. Handles new connections,
/// drops from various connections, as well as ways to distribute new tasks via
/// (write) as well as get completed tasks via (read). You can also specify
/// specific clients to send data to. There is also an init method. Connection
/// errors are handled and given as errors either to the specified readDrops or
/// if used with a specific connection, an error is given there. There is a
/// specific handler for each client function.
#[allow(unused_imports)]
use std::{error::Error, sync::mpsc, thread};
use tokio::{self, runtime::Builder, sync::broadcast, sync::mpsc as tmpsc};

/// TODO: Finish implementing the Server structure
///
/// Publicly exposed server struct that represents a server instance.
pub struct Server {
    status: ServerStatus,
    close_tx: broadcast::Sender<()>,
    read_client_tx: tmpsc::Sender<Message>,
}

/// Status of the server
enum ServerStatus {
    Running,
    NotStarted,
    NotRunning,
    Closed,
}
/// TODO: Implement a generic message type that the server can handle,
/// hopefully at runtime.
///
/// Message structure for interfacing with various reads, writes to and from
/// the clients
struct Message;
#[allow(dead_code)]

/// TODO:Finish creating the client connection structure, that will hold
/// various important information about the client
struct ClientConn {
    receiver: tmpsc::Receiver<Message>,
}

/// TODO: Finish defining the listener state
struct ListenerState {}

/// TODO: Finish defining the server state
struct ServerState {
    close_rx: broadcast::Receiver<()>,
    read_client_rx: tmpsc::Receiver<Message>,
}

/// ===========================================================================
/// STRUCTS AND TYPES ^  FUNCTIONS AND METHODS v
/// ===========================================================================

/// TODO:Since creating a server spawns resources and  threads that we need to
/// clean up afterwards, If the server is still running, we must clean up
/// the resources.
impl Drop for Server {
    fn drop(&mut self) {}
}
#[allow(dead_code)]
impl Server {
    /// Creates new server instance.
    pub fn new() -> Result<Server, Box<dyn Error>> {
        let (read_client_tx, mut read_client_rx) = tmpsc::channel(1);
        let (close_tx, close_rx) = broadcast::channel(1);
        let server_state = ServerState {
            close_rx: close_rx,
            read_client_rx: read_client_rx,
        };

        // NOTE: New thread entrypoint: runs the server separately to maximize
        // server efficiency
        std::thread::spawn(move || {
            let Ok(rt) = Builder::new_multi_thread()
                .worker_threads(2) // NOTE: Change as required
                .build()
            else {
                return;
            };
            rt.block_on(server_state.serve());
        });

        Ok(Server {
            status: ServerStatus::NotStarted,
            close_tx: close_tx,
            read_client_tx: read_client_tx,
        })
    }

    /* TODO: Finish these interfacing functions
    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn read(&self) -> Result<Message, Box<dyn Error>> {
        Ok(Message {})
    }

    pub fn write(&self, message: Message) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn read_close(&self) -> Result<Vec<u32>, Box<dyn Error>> {
        Ok(vec![])
    }
    pub async fn read_id(&self, conn_id: u32) -> Result<Message, Box<dyn Error>> {
        Ok(Message {})
    }

    pub async fn write_id(&self, conn_id: u32, message: Message) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn is_valid(&self, conn_id: u32) -> Result<bool, Box<dyn Error>> {
        Ok(false)
    }

    pub fn client_list(&self) -> Result<Vec<u32>, Box<dyn Error>> {
        Ok(vec![])
    }

    pub fn close(&self) -> Result<usize, Box<dyn Error>> {
        let k = self.close_tx.send(())?;
        Ok(k)
    }
    */
}

/// TODO: Finish implementing the server state
///
/// serve is the main loop that handles various events atomically, preventing any dataraces.
/// It interacts with numerous other threads, including ones that store various client
/// interaction data, the main listening loop for handling new connections, and also
/// interacting with client functions as well. Only Inner is capable of accessing the
/// inner server as well.
impl ServerState {
    async fn serve(mut self) {
        loop {
            tokio::select! {
                value = self.read_client_rx.recv() => (),
                _ = self.close_rx.recv() => return,

            }
        }
    }
}

/// TODO: Finish implementing the listener state.
impl ListenerState {
    async fn listen(mut self) {}
}

fn read_to_client(client_id: u32) {}

fn write_to_client() {}
