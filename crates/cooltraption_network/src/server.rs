use std::collections::HashSet;

use crate::common::NetworkingEngine;
pub use message_io::network::{Endpoint, NetEvent, Transport};
pub use message_io::node::{self, NodeEvent, NodeHandler, NodeListener};

pub struct ServerNetworkingEngine {
    pub handler: NodeHandler<()>,
}

impl ServerNetworkingEngine {
    pub fn new(handler: NodeHandler<()>) -> Self {
        Self { handler }
    }
}

impl ServerNetworkingEngine {}

pub fn listen(handler: &NodeHandler<()>, port: u32) {
    handler
        .network()
        .listen(Transport::Tcp, format!("0.0.0.0:{}", port));
}

pub fn run_event_handler(
    listener: NodeListener<()>,
    event_handler: impl FnMut(NodeEvent<()>),
) {
    listener.for_each(event_handler);
}

#[derive(Default, Debug)]
pub struct NetworkState<'a> {
    pub connected_clients: HashSet<Endpoint>,
    pub sent_messages: Vec<(Endpoint, String)>,
    pub current_event: Option<NodeEvent<'a, ()>>,
}
