use cooltraption_common::events::EventHandler;
use message_io::{network::Endpoint, node::NodeEvent};
use std::collections::HashSet;

use crate::server::Signal;

#[derive(Default, Debug)]
pub struct ClientStorage {
    pub connected_clients: HashSet<Endpoint>,
}

#[derive(Default, Debug)]
pub struct ClientStorageEventHandler {
    client_storage: ClientStorage,
}

impl<'a> EventHandler<NodeEvent<'a, Signal>> for ClientStorageEventHandler {
    fn handle_event(&mut self, event: &NodeEvent<Signal>) {
        match event {
            NodeEvent::Network(network_event) => match network_event {
                message_io::network::NetEvent::Accepted(endpoint, _) => {
                    self.client_storage.connected_clients.insert(*endpoint);
                }
                message_io::network::NetEvent::Disconnected(endpoint) => {
                    self.client_storage.connected_clients.remove(&endpoint);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
