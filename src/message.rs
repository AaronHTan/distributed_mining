/// This file handles the message bus, which is a performant message handling
/// structure that can handle arbitrary messages
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// TODO: Implement a generic message type that the server can handle,
/// hopefully at runtime.
///
/// MessageBus structure for interfacing with various reads, writes to and from
/// the clients
#[derive(Decode, Encode, Deserialize, Serialize, Debug)]
pub enum MessageBus {
    Connect {
        addr: SocketAddr,
    },
    Message {
        addr: SocketAddr,
        message_id: usize,
        len: usize,
        message: Box<Vec<u8>>,
    },
    Ack {
        addr: SocketAddr,
        id: usize,
    },
    CAck {
        addr: SocketAddr,
        id: usize,
    },
}

impl MessageBus {
    // Easy way to get the address
    pub fn addr(&self) -> &SocketAddr {
        match self {
            MessageBus::Connect { addr }
            | MessageBus::Message { addr, .. }
            | MessageBus::Ack { addr, .. }
            | MessageBus::CAck { addr, .. } => addr,
        }
    }
}
