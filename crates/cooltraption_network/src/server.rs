use std::collections::HashSet;

use crate::common::NetworkingEngine;
use cooltraption_common::events::{EventHandler, EventPublisher};
use message_io::network::ResourceId;
pub use message_io::network::{Endpoint, NetEvent, Transport};
pub use message_io::node::{self, NodeEvent, NodeHandler, NodeListener};

pub struct ServerNetworkingEngine {
}

impl ServerNetworkingEngine {
    pub fn run<T>(&mut self, port: u16, mut node_event_publisher: EventPublisher<(NodeEvent<Signal>, Context)>)
    {
        let (mut handler, listener) = node::split();
        handler
            .network()
            .listen(Transport::Tcp, format!("0.0.0.0:{}", port))
            .expect("The port to be free");


        let node_event_handler = move |node_event: NodeEvent<Signal>| {
            let context = Context{ node_handler: &mut handler };
            node_event_publisher.publish(&(node_event, context))
        };

        listener.for_each(node_event_handler);
    }
}

pub struct Context<'a> {
    pub node_handler: &'a mut NodeHandler<Signal>,
}

impl ServerNetworkingEngine {}

pub fn run_event_handler(
    listener: NodeListener<Signal>,
    event_handler: impl FnMut(NodeEvent<Signal>),
) {
    listener.for_each(event_handler);
}


#[derive(Default, Debug)]
pub struct MessageStorage {
    pub sent_messages: Vec<(Endpoint, String)>,
}

pub enum Signal {
    DisconnectClient(Endpoint),
}
