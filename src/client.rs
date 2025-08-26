/// This is the client implementation of the bitcoin client. It will attempt a
/// connection with the server. If it doesn't connect within a certain amount
/// of time, it will fail. The purpose of the client is to be able to read and
/// write from the server. If the server is not responsive, it will also
/// disconnect
use std::{
    error::Error,
    io::{Error as IoError, ErrorKind as IoErrorKind},
    net::{SocketAddr, UdpSocket},
};
use tokio::{
    self,
    net::UdpSocket as UdpSocket_T,
    runtime::Builder,
    sync::{broadcast, mpsc as tmpsc, oneshot, watch},
};

pub struct ClientBuild {
    server_port: Option<String>,
}
pub struct Client {
    port: Option<String>,
}

struct ClientState {}

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
        let socket = UdpSocket::bind(server_addr)?;
        // TODO: make sure server is actually connected before doing anything
        let client_state = ClientState {};

        let relay_state = RelayState {};

        std::thread::spawn(move || {
            let Ok(rt) = Builder::new_multi_thread()
                .worker_threads(1) // NOTE: Change as required
                .build()
            else {
                return;
            };
            let r = rt.block_on({
                tokio::spawn(async move {
                    let res = relay_state.listen().await;
                    match res {
                        Err(e) => eprintln!("Error found in {}", e),
                        _ => (),
                    };
                });

                client_state.client()
            });
        });

        Ok(Client { port: None })
    }
}

impl Client {}

impl ClientState {
    async fn client(mut self) {
        loop {}
    }
}

impl RelayState {
    async fn listen(mut self) -> Result<(), Box<dyn Error>> {
        loop {}
        Ok(())
    }
}
