use std::fmt::format;

use message_io::{
    network::{NetEvent, Transport},
    node,
};

use crate::server::ServerNetworkingEngine;
use super::common::NetworkingEngine;

pub struct RelayServer {
    server_networking_engine: ServerNetworkingEngine,
}
