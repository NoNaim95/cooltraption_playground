use std::{
    marker::PhantomData,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::connection::Connection;
use crate::packets::Packet;
use bimap::BiMap;

use super::*;
use message_io::{
    network::Endpoint,
    node::{NodeEvent, NodeHandler},
};
use serde::{de::DeserializeOwned, Serialize};

pub struct MessageIoAdapter<P> {
    connections: BiMap<Connection, Endpoint>,
    node_handler: NodeHandler<Signal>,
    _phantom: PhantomData<fn() -> P>,
}

impl<P> NetworkInterface<P> for MessageIoAdapter<P>
where
    P: Serialize + DeserializeOwned,
{
    fn send_packet(&self, packet: Packet<P>, connection: &Connection) {
        let endpoint = self.connections.get_by_left(connection).unwrap();
        self.node_handler.network().send(
            *endpoint,
            serde_json::to_string(&packet).unwrap().as_bytes(),
        );
    }

    fn connections(&self) -> Vec<Connection> {
        self.connections.left_values().copied().collect()
    }

    fn disconnect(&mut self, id: &Connection) {
        let resource_id = self.connections.get_by_left(id).unwrap().resource_id();
        self.node_handler.network().remove(resource_id);
        self.connections.remove_by_left(id);
    }

    fn stop_listener(&mut self) {
        self.node_handler.stop();
    }
}

impl<T> MessageIoAdapter<T> {
    pub fn new(node_handler: NodeHandler<Signal>) -> Self {
        Self {
            connections: Default::default(),
            node_handler,
            _phantom: PhantomData,
        }
    }

    fn add_endpoint(&mut self, endpoint: Endpoint) {
        self.connections
            .insert(Connection::new(endpoint.addr()), endpoint);
    }

    fn remove_endpoint(&mut self, endpoint: &Endpoint) {
        self.connections.remove_by_right(endpoint);
    }

    pub(super) fn node_handler(&self) -> NodeHandler<Signal> {
        self.node_handler.clone()
    }

    pub(super) fn apply_node_event(
        &mut self,
        message: &NodeEvent<'_, Signal>,
    ) -> NetworkInterfaceEvent<T>
    where
        T: DeserializeOwned,
    {
        if let NodeEvent::Network(net_event) = message {
            let network_state_event: NetworkInterfaceEvent<T> = match net_event {
                message_io::network::NetEvent::Connected(endpoint, established) => {
                    if !established {
                        panic!("connection failed");
                    }
                    self.add_endpoint(*endpoint);
                    NetworkInterfaceEvent::Connected(
                        *self.connections.get_by_right(endpoint).unwrap(),
                    )
                }
                message_io::network::NetEvent::Accepted(endpoint, _) => {
                    self.add_endpoint(*endpoint);
                    NetworkInterfaceEvent::Accepted(
                        *self.connections.get_by_right(endpoint).unwrap(),
                    )
                }
                message_io::network::NetEvent::Message(endpoint, message) => {
                    let connection = *self.connections.get_by_right(endpoint).unwrap();
                    let packet = serde_json::from_slice::<Packet<T>>(message).unwrap();
                    NetworkInterfaceEvent::Message(connection, packet)
                }
                message_io::network::NetEvent::Disconnected(endpoint) => {
                    let connection = *self.connections.get_by_right(endpoint).unwrap();
                    self.remove_endpoint(endpoint);
                    NetworkInterfaceEvent::Disconnected(connection)
                }
            };
            network_state_event
        } else {
            unimplemented!();
        }
    }
}
pub type ConcurrentMessageIoAdapter<T> = Arc<Mutex<MessageIoAdapter<T>>>;
pub type NetworkStateEventHandler<T> =
    Box<dyn FnMut(&NetworkInterfaceEvent<T>, &mut MutexGuard<MessageIoAdapter<T>>) + Send>;
