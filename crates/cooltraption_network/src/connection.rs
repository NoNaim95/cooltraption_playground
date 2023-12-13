use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ConnectionId(Uuid);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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
