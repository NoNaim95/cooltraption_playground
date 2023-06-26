use bimap::BiMap;
use message_io::{
    network::Endpoint,
    node::{NodeEvent, NodeHandler},
};
use std::{net::SocketAddr, sync::{Arc, Mutex, MutexGuard}};
use uuid::Uuid;

use crate::{packets::Packet, server::Signal};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ConnectionId(Uuid);

#[allow(dead_code)]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

#[allow(dead_code)]
pub struct EndpointConnection {
    id: ConnectionId,
    endpoint: Endpoint,
}

pub trait RawMessage{}
impl<'a, T> RawMessage for NodeEvent<'a, T>{}

#[derive(Clone)]
pub struct NetworkStateImpl {
    connections: BiMap<Connection, Endpoint>,
    node_handler: NodeHandler<Signal>,
}

impl NetworkStateImpl {
    pub fn new(node_handler: NodeHandler<Signal>) -> Self {
        Self {
            connections: Default::default(),
            node_handler,
        }
    }

    fn add_endpoint(&mut self, endpoint: Endpoint) {
        self.connections
            .insert(Connection::new(endpoint.addr()), endpoint);
    }

    fn remove_endpoint(&mut self, endpoint: &Endpoint) {
        self.connections.remove_by_right(endpoint);
    }

    fn apply_raw_message(&mut self, message: &NodeEvent<'_, Signal>) -> NetworkStateEvent {
        let mut network_state_event: Option<NetworkStateEvent> = None;

        if let NodeEvent::Network(net_event) = message {
            match net_event {
                message_io::network::NetEvent::Connected(endpoint, _) => {
                    println!("Connected to Server!");
                    self.add_endpoint(endpoint.clone());
                    network_state_event = Some(NetworkStateEvent::Connected(
                        self.connections
                            .get_by_right(&endpoint)
                            .unwrap()
                            .clone(),
                    ));
                }
                message_io::network::NetEvent::Accepted(endpoint, _) => {
                    println!("Client Connected!");
                    self.add_endpoint(endpoint.clone());
                    network_state_event = Some(NetworkStateEvent::Accepted(
                        self.connections
                            .get_by_right(&endpoint)
                            .unwrap()
                            .clone(),
                    ));
                }
                message_io::network::NetEvent::Message(endpoint, message) => {
                    println!("Message received!");
                    let connection = self
                        .connections
                        .get_by_right(&endpoint)
                        .unwrap()
                        .clone();
                    let packet = serde_yaml::from_slice::<Packet<()>>(&message).unwrap();
                    network_state_event = Some(NetworkStateEvent::Message(connection, packet));
                }
                message_io::network::NetEvent::Disconnected(endpoint) => {
                    println!("Client Disconnected!");
                    let connection = self
                        .connections
                        .get_by_right(&endpoint)
                        .unwrap()
                        .clone();
                    self.remove_endpoint(&endpoint);
                    network_state_event = Some(NetworkStateEvent::Disconnected(connection))
                }
            }
            return network_state_event.unwrap();
        }
        else {
            unimplemented!();
        }
    }

    fn send_packet(&self, packet: Packet<()>, connection: &Connection) {
        let endpoint = self.connections.get_by_left(connection).unwrap();
        self.node_handler.network().send(endpoint.clone(), serde_yaml::to_string(&packet).unwrap().as_bytes());
    }

    fn connections(&self) -> Vec<&Connection> {
        self.connections.left_values().collect()
    }

    fn disconnect(&mut self, id: Connection) {
        let resource_id = self.connections.get_by_left(&id).unwrap().resource_id();
        self.node_handler.network().remove(resource_id);
        self.connections.remove_by_left(&id);
    }
}

pub enum NetworkStateEvent {
    Connected(Connection),
    Accepted(Connection),
    Disconnected(Connection),
    Message(Connection, Packet<()>),
}

pub struct NodeEventHandler<F>
where
    F: FnMut(& NetworkStateEvent, &mut MutexGuard<NetworkStateImpl>),
{
    network_state: Arc<Mutex<NetworkStateImpl>>,
    network_state_publisher: F,
}

impl<F> NodeEventHandler<F>
where
    F: FnMut(& NetworkStateEvent, &mut MutexGuard<NetworkStateImpl>),
{
    pub fn new(network_state: NetworkStateImpl, network_state_publisher: F) -> Self {
        Self {
            network_state: Arc::new(Mutex::new(network_state)),
            network_state_publisher,
        }
    }

    pub fn handle_node_event(&mut self, event: NodeEvent<'_, Signal>) {
        let mut network_state_lock = self.network_state.lock().unwrap();
        let network_state_event = network_state_lock.apply_raw_message(&event);
        (self.network_state_publisher)(&network_state_event, &mut network_state_lock);
    }
}
