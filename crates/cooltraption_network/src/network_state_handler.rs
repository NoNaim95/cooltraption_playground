use cooltraption_common::events::MutEventHandler;
use message_io::node::{StoredNetEvent, StoredNodeEvent};

use crate::{
    node_event_handler::NetworkState,
    server::Signal, events::MutEvent,
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

