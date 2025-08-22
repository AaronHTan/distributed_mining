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
    net::{TcpListener, TcpStream},
    runtime::Builder,
    sync::{broadcast, mpsc as tmpsc, oneshot},
};

const LOCALHOST_PORT: &str = "127.0.0.1:8080";

/// server created struct, doesn't hold any information currently
pub struct ServerCreated {
    port: Option<String>,
}

/// server running struct, allows communication with the actual serverstate
pub struct ServerRunning {
    close_tx: broadcast::Sender<()>,
    read_client_tx: tmpsc::Sender<MessageBus>,
}

/// closed server struct, allows viewing of details in server data.
pub struct ServerClosed {}

/// TODO: Implement a generic message type that the server can handle,
/// hopefully at runtime.
///
/// MessageBus structure for interfacing with various reads, writes to and from
/// the clients
pub struct MessageBus;

/// TODO:Finish creating the client connection structure, that will hold
/// various important information about the client
struct ClientConn {
    receiver: tmpsc::Receiver<MessageBus>,
}

struct ClientConnBus {
    stream: TcpStream,
    addr: SocketAddr,
}

/// TODO: Finish defining the listener state
struct ListenerState {
    port: String,
    close_rx: broadcast::Receiver<()>,
    error_tx: tmpsc::Sender<()>,
    new_conn_tx: tmpsc::Sender<ClientConnBus>,
}

/// TODO: Finish defining the server state
struct ServerState {
    start_rx: oneshot::Receiver<()>,
    close_rx: broadcast::Receiver<()>,
    listener_error_rx: tmpsc::Receiver<()>,
    read_client_rx: tmpsc::Receiver<MessageBus>,
    new_conn_rx: tmpsc::Receiver<ClientConnBus>,
    port: String,
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
        let (listener_error_tx, listener_error_rx) = tmpsc::channel(1);
        let (read_client_tx, read_client_rx) = tmpsc::channel(1);
        let (new_conn_tx, new_conn_rx) = tmpsc::channel(1);

        let port = self.port.unwrap_or(String::from(LOCALHOST_PORT));

        let listener_state = ListenerState {
            port: port.clone(),
            close_rx: close_tx.subscribe(),
            error_tx: listener_error_tx,
            new_conn_tx: new_conn_tx,
        };
        let server_state = ServerState {
            start_rx: start_rx,
            close_rx: close_tx.subscribe(),
            listener_error_rx: listener_error_rx,
            read_client_rx: read_client_rx,
            new_conn_rx: new_conn_rx,
            port: String::from("127.0.0.1:8080"),
        };

        let server_running = ServerRunning {
            close_tx: close_tx,
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

            /// NOTE: this should be changed to just fail the entire server structure, not panic
            let r = rt.block_on({
                tokio::spawn(async move {
                    let res = listener_state.listen().await;
                    match res {
                        Err(e) => eprintln!("Error found in {}", e),
                        _ => (),
                    };
                });

                server_state.serve()
            });
        });

        Ok(server_running)
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl ServerRunning {
    pub fn read(&self) -> Result<MessageBus, Box<dyn Error>> {
        Ok(MessageBus {})
    }

    pub fn write(&self, message: MessageBus) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn read_close(&self) -> Result<Vec<u32>, Box<dyn Error>> {
        Ok(vec![])
    }

    pub async fn read_id(&self, conn_id: u32) -> Result<MessageBus, Box<dyn Error>> {
        Ok(MessageBus {})
    }

    pub async fn write_id(&self, conn_id: u32, message: MessageBus) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn is_valid(&self, conn_id: u32) -> Result<bool, Box<dyn Error>> {
        Ok(false)
    }

    pub fn client_list(&self) -> Result<Vec<u32>, Box<dyn Error>> {
        Ok(vec![])
    }

    /// Transition from Running to Closed state
    pub fn close(self) -> ServerClosed {
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
    async fn serve(mut self) -> Result<(), Box<dyn Error>> {
        loop {
            tokio::select! {
                biased;
                value = self.read_client_rx.recv() => (),
                _ = self.close_rx.recv() => return Ok(()),
            }
        }
    }
}

impl ListenerState {
    /// listen is the main listening loop that uses ListenerState to store its state.
    /// Recoverable errors (whenever clients abort connection) are ignored, otherwise
    /// it panics
    async fn listen(mut self) -> Result<(), Box<dyn Error>> {
        let listener = match TcpListener::bind(&self.port).await {
            Ok(x) => x,
            Err(e) => {
                self.error_tx.send(()).await?;
                return Err(Box::new(e) as Box<dyn Error>);
            }
        };
        loop {
            tokio::select! {
                close_signal = self.close_rx.recv() =>
                    return close_signal.map_err(|error| Box::new(error) as Box<dyn Error>),
                accept_res = listener.accept() => {
                    match accept_res {
                        Ok((socket, addr)) => self.handle_new_conn(socket, addr).await?,
                        Err(e) => self.handle_error(e).await?,
                    }
                }
            }
        }
    }

    /// simple function to create the client connection message bus and send it to the
    /// main server loop. This will block until the main server loop can accept, although
    /// it should be quick, given that it is prioritized. in the serve loop.
    async fn handle_new_conn(
        &mut self,
        stream: TcpStream,
        addr: SocketAddr,
    ) -> Result<(), Box<dyn Error>> {
        let conn_info = ClientConnBus {
            stream: stream,
            addr: addr,
        };
        self.new_conn_tx.send(conn_info).await?;
        Ok(())
    }

    /// Simple error handler that doesn't fail the entire thing for certain accept errors.
    async fn handle_error(&mut self, error: IoError) -> Result<(), IoError> {
        match error.kind() {
            IoErrorKind::ConnectionAborted | IoErrorKind::Interrupted | IoErrorKind::WouldBlock => {
                Ok(())
            }

            _ => {
                self.error_tx
                    .try_send(())
                    .unwrap_or_else(|e| eprintln!("Also got error while sending on error_tx{e}"));
                Err(error)
            }
        }
    }
}
