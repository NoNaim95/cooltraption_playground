use cooltraption_common::events::{MutEventHandler, MutEventPublisher};
use message_io::{
    network::Endpoint,
    node::{NodeHandler, StoredNetEvent, StoredNodeEvent},
};
use std::collections::HashSet;

use crate::server::{Context, Signal};

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

#[derive(Default, Debug)]
pub struct NetworkStateEventHandler<'a> {
    network_state: NetworkState,
    event_publisher: MutEventPublisher<'a, (NetworkState, StoredNodeEvent<Signal>, Context)>,
}

impl<'a> NetworkStateEventHandler<'a> {
    pub fn add_handler(
        &mut self,
        event_handler: impl MutEventHandler<(NetworkState, StoredNodeEvent<Signal>, Context)> + 'a,
    ) {
        self.event_publisher.add_event_handler(event_handler);
    }
}

impl<'a> MutEventHandler<(StoredNodeEvent<Signal>, Context)> for NetworkStateEventHandler<'a> {
    fn handle_event(&mut self, event: &mut (StoredNodeEvent<Signal>, Context)) {
        let (stored_node_event, context) = event;
        if let StoredNodeEvent::Network(network_event) = stored_node_event {
            match network_event {
                StoredNetEvent::Accepted(endpoint, _) => {
                    self.network_state.connected_clients.insert(*endpoint);
                }
                StoredNetEvent::Disconnected(endpoint) => {
                    println!("Client Disconnected");
                    self.network_state.connected_clients.remove(endpoint);
                }
                StoredNetEvent::Message(endpoint, msg) => self
                    .network_state
                    .sent_messages
                    .push((*endpoint, (*msg).clone())),
                _ => {}
            }
        }
        self.event_publisher.publish(&mut (
            self.network_state.clone(),
            (*stored_node_event).clone(),
            (*context).clone(),
        ));
    }
}
