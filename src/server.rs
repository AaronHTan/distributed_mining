/// Distibuted Server for handling Mining Operations. Handles new connections,
/// drops from various connections, as well as ways to distribute new tasks via
/// (write) as well as get completed tasks via (read). You can also specify
/// specific clients to send data to. There is also an init method. Connection
/// errors are handled and given as errors either to the specified readDrops or
/// if used with a specific connection, an error is given there. There is a
/// specific handler for each client function.
use std::{
    error::Error,
    io::{Error as IoError, ErrorKind as IoErrorKind},
    net::SocketAddr,
};
use tokio::{
    self,
    net::UdpSocket as UdpSocket_T,
    runtime::Builder,
    sync::{broadcast, mpsc as tmpsc, oneshot},
};

use super::message::MessageBus;

const LOCALHOST_PORT: &str = "127.0.0.1:8080";
const UDP_MAX_SIZE: usize = 65507;
const CONFIG: bincode::config::Configuration = bincode::config::standard();

/// server created struct, doesn't hold any information currently
pub struct ServerCreated {
    port: Option<String>,
}

/// server running struct, allows communication with the actual serverstate
#[allow(dead_code)]
pub struct ServerRunning {
    close_tx: tmpsc::Sender<()>,
    read_client_tx: tmpsc::Sender<MessageBus>,
}

/// closed server struct, allows viewing of details in server data.
pub struct ServerClosed {}

/// TODO:Finish creating the client connection structure, that will hold
/// various important information about the client
#[allow(dead_code)]
struct ClientQueue {
    receiver: tmpsc::Receiver<MessageBus>,
    close_rx: broadcast::Receiver<()>,
}

#[allow(dead_code)]
struct ClientInfo {
    addr: SocketAddr,
    sender: tmpsc::Sender<MessageBus>,
    current_id: usize,
}

/// TODO: Finish defining the listener state
struct ListenerState {
    addr: SocketAddr,
    close_rx: broadcast::Receiver<()>,
    error_tx: tmpsc::Sender<()>,
    new_message_tx: tmpsc::Sender<MessageBus>,
}

/// TODO: Finish defining the server state
#[allow(dead_code)]
struct ServerState {
    start_rx: oneshot::Receiver<()>,
    close_tx: broadcast::Sender<()>,
    close_command_rx: tmpsc::Receiver<()>,
    read_client_rx: tmpsc::Receiver<MessageBus>,
    new_conn_rx: tmpsc::Receiver<MessageBus>,
    port: String,
    addr: SocketAddr,
}

/// ===========================================================================
/// STRUCTS AND TYPES ^  FUNCTIONS AND METHODS v
/// ===========================================================================

#[allow(dead_code)]
#[allow(unused_variables)]
impl ServerCreated {
    pub fn builder() -> ServerCreated {
        ServerCreated { port: None }
    }

    /// Creates new server instance in the Created state.
    pub fn run(self) -> Result<ServerRunning, Box<dyn Error>> {
        let (start_tx, start_rx) = oneshot::channel();
        let (close_tx, _) = broadcast::channel(1);
        let (close_command_tx, close_command_rx) = tmpsc::channel(1);
        let (read_client_tx, read_client_rx) = tmpsc::channel(1);
        let (new_message_tx, new_conn_rx) = tmpsc::channel(1);

        let port = self.port.unwrap_or(String::from(LOCALHOST_PORT));
        let addr: SocketAddr = (String::from("0.0.0.0:") + &port).parse()?;

        let listener_state = ListenerState {
            addr: addr.clone(),
            close_rx: close_tx.subscribe(),
            error_tx: close_command_tx.clone(),
            new_message_tx: new_message_tx,
        };
        let server_state = ServerState {
            start_rx: start_rx,
            close_tx: close_tx,
            close_command_rx: close_command_rx,
            read_client_rx: read_client_rx,
            new_conn_rx: new_conn_rx,
            port: String::from("127.0.0.1:8080"),
            addr: addr,
        };

        let server_running = ServerRunning {
            close_tx: close_command_tx,
            read_client_tx: read_client_tx,
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

            // NOTE: this should be changed to just fail the entire server structure, not panic
            let r = rt.block_on({
                tokio::spawn(async move {
                    listener_state.listen_wrapper().await;
                });

                server_state.serve_wrapper()
            });
        });

        Ok(server_running)
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl ServerRunning {
    pub async fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(vec![])
    }

    pub async fn write(&mut self, message: &[u8]) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub async fn read_close(&mut self) -> Result<Vec<u32>, Box<dyn Error>> {
        Ok(vec![])
    }

    pub async fn read_id(&mut self, conn_id: u32) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(vec![])
    }

    pub async fn write_id(
        &mut self,
        conn_id: u32,
        message: MessageBus,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub async fn is_valid(&mut self, conn_id: u32) -> Result<bool, Box<dyn Error>> {
        Ok(false)
    }

    pub async fn client_list(&mut self) -> Result<Vec<u32>, Box<dyn Error>> {
        Ok(vec![])
    }

    /// Transition from Running to Closed state
    pub async fn close(self) -> ServerClosed {
        let _ = self.close_tx.send(());

        ServerClosed {}
    }
}

#[allow(dead_code)]
impl ServerClosed {
    /// Methods available only when the server is closed
    /// For now, closed servers cannot be restarted
    pub fn is_closed(&self) -> bool {
        true
    }
}

/// TODO: Finish implementing the server state
///
/// serve is the main loop that handles various events atomically, preventing any dataraces.
/// It interacts with numerous other threads, including ones that store various client
/// interaction data, the main listening loop for handling new connections, and also
/// interacting with client functions as well. Only Inner is capable of accessing the
/// inner server as well.
#[allow(unused_variables)]
impl ServerState {
    async fn serve_wrapper(self) {
        if let Err(e) = self.serve().await {
            eprintln!("Error {e} occurred in serve, returning");
        }
    }
    async fn serve(mut self) -> Result<(), Box<dyn Error>> {
        self.start_rx.blocking_recv()?;
        loop {
            tokio::select! {
                new_conn = self.new_conn_rx.recv() => {

            }
                value = self.read_client_rx.recv() => (),
                _ = self.close_command_rx.recv() => {
                    return Ok(());
                }
            }
        }
    }
}

impl ListenerState {
    async fn listen_wrapper(mut self) {
        if let Err(e) = self.listen().await {
            eprintln!("Error {e} occurred in the listen loop");
            self.error_tx
                .try_send(())
                .unwrap_or_else(|e| eprintln!("Also got error while sending on error_tx{e}"));
        }
    }
    /// listen is the main listening loop that uses handle_message to store its state.
    /// Recoverable errors (whenever clients abort connection) are ignored, otherwise
    /// it panics
    async fn listen(&mut self) -> Result<(), Box<dyn Error>> {
        let udp_socket = UdpSocket_T::bind(self.addr).await?;
        let mut buf = [0; UDP_MAX_SIZE];
        loop {
            tokio::select! {
                close_signal = self.close_rx.recv() =>
                    return close_signal.map_err(|error| Box::new(error) as Box<dyn Error>),
                accept_res = udp_socket.recv_from(&mut buf) => {
                    match accept_res {
                        Ok((len, addr)) => self.handle_message(addr, &buf[..len]).await?,
                        Err(e) => self.handle_error(e).await?,
                    }
                }
            }
        }
    }

    /// simple function to create the client connection message bus and send it to the
    /// main server loop. This will block until the main server loop can accept, although
    /// it should be quick, given that it is prioritized. in the serve loop.
    async fn handle_message(&mut self, addr: SocketAddr, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        let (message, _): (MessageBus, usize) = bincode::decode_from_slice(buf, CONFIG)?;
        if message.addr() != &addr {
            return Ok(());
        }
        self.new_message_tx.send(message).await?;
        Ok(())
    }

    /// Simple error handler that doesn't fail the entire thing for certain accept errors.
    async fn handle_error(&mut self, error: IoError) -> Result<(), IoError> {
        match error.kind() {
            IoErrorKind::ConnectionAborted | IoErrorKind::Interrupted | IoErrorKind::WouldBlock => {
                Ok(())
            }

            _ => Err(error),
        }
    }
}
