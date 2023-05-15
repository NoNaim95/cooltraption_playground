use cooltraption_common::events::{
    EventHandler, EventPublisher, MutEventHandler, MutEventPublisher,
};
use message_io::{
    network::Endpoint,
    node::{NodeEvent, NodeHandler, StoredNetEvent, StoredNodeEvent},
};
use std::collections::HashSet;

use crate::{
    events::Event,
    server::Signal,
};

#[derive(Default, Debug, Clone)]
pub struct NetworkState {
    pub connected_clients: HashSet<Endpoint>,
    pub sent_messages: Vec<(Endpoint, Vec<u8>)>,
}

impl NetworkState {
    pub fn disconnect_client(&mut self, node_handler: NodeHandler<Signal>, endpoint: Endpoint) {
        node_handler.network().remove(endpoint.resource_id());
        self.connected_clients.remove(&endpoint);
    }
}

struct NodeEventHandler<'a> {
    network_state: NetworkState,
    network_state_publisher: EventPublisher<'a, Event<'a, NodeEvent<'a, Signal>, NetworkState>>,
}

impl<'a> EventHandler<Event<'a, NodeEvent<'a, Signal>>> for NodeEventHandler<'a> {
    fn handle_event(&mut self, event: &Event<'a, NodeEvent<'a, Signal>>) {
        if let NodeEvent::Network(net_event) = event.payload() {
            match net_event {
                message_io::network::NetEvent::Connected(_, _) => {
                    println!("")
                },
                message_io::network::NetEvent::Accepted(_, _) =>{
                    println!("Client Connected!, setting network_state accordingly!");
                },
                message_io::network::NetEvent::Message(_, _) =>{
                    println!("Message received!, setting network_state accordingly!");
                },
                message_io::network::NetEvent::Disconnected(_) =>{
                    println!("Client Disconnected!, setting network_state accordingly!");
                },
            }
        }
        self.network_state_publisher.publish(&Event::new(&event.payload(), &self.network_state));
    }
}
