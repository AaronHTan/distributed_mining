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
use tokio::{
    self,
    net::{TcpListener, TcpStream},
    runtime::Builder,
    sync::{broadcast, mpsc as tmpsc, oneshot, watch},
};

pub struct ClientBuild {}
pub struct Client {
    port: Option<String>,
}

struct ClientState {}

struct ListenerState {}
impl ClientBuild {
    pub fn build() -> ClientBuild {
        ClientBuild {}
    }

    pub fn connect(self) -> Result<Client, Box<dyn Error>> {
        let client_state = ClientState {};

        let listener_state = ListenerState {};

        std::thread::spawn(move || {
            let Ok(rt) = Builder::new_multi_thread()
                .worker_threads(2) // NOTE: Change as required
                .build()
            else {
                return;
            };
            let r = rt.block_on({
                tokio::spawn(async move {
                    let res = listener_state.listen().await;
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

impl ListenerState {
    async fn listen(mut self) -> Result<(), Box<dyn Error>> {
        loop {}
        Ok(())
    }
}
