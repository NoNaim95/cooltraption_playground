use message_io::network::Endpoint;
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ConnectionId(Uuid);

#[allow(dead_code)]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Connection {
    id: ConnectionId,
    socket_addr: SocketAddr,
}

impl Connection {
    pub fn new(socket_addr: SocketAddr) -> Self {
        Self {
            id: ConnectionId(Uuid::new_v4()),
            socket_addr,
        }
    }
}

#[allow(dead_code)]
pub struct EndpointConnection {
    id: ConnectionId,
    endpoint: Endpoint,
}
