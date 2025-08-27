/// This is the client implementation of the bitcoin client. It will attempt a
/// connection with the server. If it doesn't connect within a certain amount
/// of time, it will fail. The purpose of the client is to be able to read and
/// write from the server. If the server is not responsive, it will also
/// disconnect
use std::{
    error::Error,
    io::{Error as IoError, ErrorKind as IoErrorKind},
    net::SocketAddr,
};
use tokio::{net::UdpSocket as UdpSocket_T, runtime::Builder, select};

use super::message::MessageBus;
const UDP_MAX_SIZE: usize = 65507;
const CONFIG: bincode::config::Configuration = bincode::config::standard();

pub struct ClientBuild {
    server_port: Option<String>,
}
pub struct Client {
    _port: Option<String>,
}

struct ClientState {
    _addr: SocketAddr,
    socket: UdpSocket_T,
}

struct RelayState {}
impl ClientBuild {
    pub fn build() -> ClientBuild {
        ClientBuild { server_port: None }
    }

    pub fn add_server(mut self, server_port: &str) -> Self {
        self.server_port = Some(String::from(server_port));
        self
    }

    pub async fn connect(self) -> Result<Client, Box<dyn Error>> {
        let server_addr: SocketAddr = self
            .server_port
            .unwrap_or(String::from("127.0.0.1:8080"))
            .parse()?;
        // Bind to any available local address, don't bind to server address
        let socket = UdpSocket_T::bind("127.0.0.1:0").await?;
        // TODO: make sure server is actually connected before doing anything
        let client_state = ClientState {
            _addr: server_addr,
            socket: socket,
        };

        let relay_state = RelayState {};

        std::thread::spawn(move || {
            let Ok(rt) = Builder::new_multi_thread()
                .worker_threads(1) // NOTE: Change as required
                .enable_all()
                .build()
            else {
                return;
            };
            let _ = rt.block_on(async move {
                tokio::spawn(async move {
                    let res = relay_state.listen().await;
                    match res {
                        Err(e) => eprintln!("Error found in {}", e),
                        _ => (),
                    };
                });

                client_state.client().await
            });
        });

        Ok(Client { _port: None })
    }
}

impl Client {
    pub async fn write(&mut self, _message: &[u8]) -> Result<(), Box<dyn Error>> {
        return Ok(());
    }
}

impl ClientState {
    async fn client(mut self) -> Result<(), Box<dyn Error>> {
        let mut buf = [0; UDP_MAX_SIZE];
        loop {
            select! {
                accept_res = self.socket.recv_from(&mut buf) => {
                    match accept_res {
                        Ok((len, addr)) => self.handle_message(addr, &buf[..len])?,
                        Err(e) => self.handle_error(e).await?,
                    }
                }

            }
        }
    }

    fn handle_message(&mut self, addr: SocketAddr, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        let (message, _): (MessageBus, usize) = bincode::decode_from_slice(buf, CONFIG)?;
        if message.addr() != &addr {
            return Ok(());
        }
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

impl RelayState {
    async fn listen(self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
