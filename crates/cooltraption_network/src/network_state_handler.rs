use cooltraption_common::events::MutEventHandler;
use message_io::node::{StoredNetEvent, StoredNodeEvent};

use crate::{
    network_state::NetworkState,
    server::{Context, Signal},
};

pub struct NetworkStateHandler {
    max_clients: usize,
}

impl NetworkStateHandler {
    pub fn new(max_clients: usize) -> Self {
        Self { max_clients }
    }
    pub fn set_max_client_num(&mut self, max_clients: usize) {
        self.max_clients = max_clients;
    }
}

impl MutEventHandler<(NetworkState, StoredNodeEvent<Signal>, Context)> for NetworkStateHandler {
    fn handle_event(&mut self, event: &mut (NetworkState, StoredNodeEvent<Signal>, Context)) {
        let (network_state, stored_node_event, context) = event;
        if let StoredNodeEvent::Network(stored_network_event) = stored_node_event {
            match stored_network_event {
                StoredNetEvent::Accepted(endpoint, _) => {
                    if network_state.connected_clients.len() > self.max_clients {
                        network_state.disconnect_client(context.node_handler.clone(), *endpoint)
                    }
                }
                StoredNetEvent::Message(endpoint, msg) => {
                    for client in &network_state.connected_clients {
                        if client != endpoint {
                            context.node_handler.network().send(*client, msg);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
