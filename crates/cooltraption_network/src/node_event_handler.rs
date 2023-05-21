use bimap::BiMap;
use cooltraption_common::events::{EventHandler, EventPublisher, MutEventPublisher};
use message_io::{
    network::Endpoint,
    node::{NodeEvent, NodeHandler},
};
use std::net::SocketAddr;
use uuid::Uuid;

use crate::{events::{Event, MutEvent}, packets::{Packet, ChatMessage}, server::Signal};

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

pub trait NetworkState<'a> {
    fn send_packet(&self, packet: Packet<()>, connection: &Connection);
    fn connections(&self) -> Vec<&Connection>;
    fn disconnect(&mut self, connection: Connection);
}

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

    pub fn connections(&self) -> &BiMap<Connection, Endpoint> {
        &self.connections
    }

    fn add_endpoint(&mut self, endpoint: Endpoint) {
        self.connections
            .insert(Connection::new(endpoint.addr()), endpoint);
    }

    fn remove_endpoint(&mut self, endpoint: &Endpoint) {
        self.connections.remove_by_right(endpoint);
    }
}

impl<'a> NetworkState<'a> for NetworkStateImpl {
    fn connections(&self) -> Vec<&Connection> {
        self.connections.left_values().collect()
    }

    fn disconnect(&mut self, id: Connection) {
        let resource_id = self.connections.get_by_left(&id).unwrap().resource_id();
        self.node_handler.network().remove(resource_id);
        self.connections.remove_by_left(&id);
    }

    fn send_packet(&self, packet: Packet<()>, connection: &Connection) {
        todo!()
    }
}

pub enum NetworkStateEvent {
    Connected(Connection),
    Accepted(Connection),
    Disconnected(Connection),
    Message(Connection, Packet<()>),
}

pub struct NodeEventHandler<'a> {
    network_state: NetworkStateImpl,
    network_state_publisher: MutEventPublisher<'a, MutEvent<'a, NetworkStateEvent, NetworkStateImpl>>,
}

impl<'a> NodeEventHandler<'a> {
    pub fn new(
        network_state: NetworkStateImpl,
        network_state_publisher: MutEventPublisher<'a, MutEvent<'a, NetworkStateEvent, NetworkStateImpl>>,
    ) -> Self {
        Self {
            network_state,
            network_state_publisher,
        }
    }

    pub fn handle_node_event(&mut self, event: NodeEvent<'_, Signal>) {
        let mut network_state_event: Option<NetworkStateEvent> = None;
        if let NodeEvent::Network(net_event) = event {
            match net_event {
                message_io::network::NetEvent::Connected(endpoint, _) => {
                    println!("Connected to Server!");
                    self.network_state.add_endpoint(endpoint);
                    network_state_event = Some(NetworkStateEvent::Connected(
                        self.network_state
                            .connections
                            .get_by_right(&endpoint)
                            .unwrap()
                            .clone(),
                    ));
                }
                message_io::network::NetEvent::Accepted(endpoint, _) => {
                    println!("Client Connected!");
                    self.network_state.add_endpoint(endpoint);
                    network_state_event = Some(NetworkStateEvent::Accepted(
                        self.network_state
                            .connections
                            .get_by_right(&endpoint)
                            .unwrap()
                            .clone(),
                    ));
                }
                message_io::network::NetEvent::Message(endpoint, message) => {
                    println!("Message received!");
                    let connection = self
                        .network_state
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
                        .network_state
                        .connections
                        .get_by_right(&endpoint)
                        .unwrap()
                        .clone();
                    self.network_state.remove_endpoint(&endpoint);
                    network_state_event = Some(NetworkStateEvent::Disconnected(connection))
                }
            }
        }
        self.network_state_publisher.publish(&mut MutEvent::new(
            &mut network_state_event.unwrap(),
            &mut self.network_state,
        ));
    }
}

impl<'a> EventHandler<Event<'a, NodeEvent<'a, Signal>>> for NodeEventHandler<'a> {
    fn handle_event(&mut self, event: &Event<'a, NodeEvent<'a, Signal>>) {
        let mut network_state_event: Option<NetworkStateEvent> = None;
        if let NodeEvent::Network(net_event) = event.payload() {
            match net_event {
                message_io::network::NetEvent::Connected(endpoint, _) => {
                    println!("Connected to Server!");
                    self.network_state.add_endpoint(*endpoint);
                    network_state_event = Some(NetworkStateEvent::Connected(
                        self.network_state
                            .connections
                            .get_by_right(endpoint)
                            .unwrap()
                            .clone(),
                    ));
                }
                message_io::network::NetEvent::Accepted(endpoint, _) => {
                    println!("Client Connected!");
                    self.network_state.add_endpoint(*endpoint);
                    network_state_event = Some(NetworkStateEvent::Accepted(
                        self.network_state
                            .connections
                            .get_by_right(endpoint)
                            .unwrap()
                            .clone(),
                    ));
                }
                message_io::network::NetEvent::Message(endpoint, message) => {
                    println!("Message received!");
                    let connection = self
                        .network_state
                        .connections
                        .get_by_right(&endpoint)
                        .unwrap()
                        .clone();
                    let packet = serde_yaml::from_slice::<Packet<()>>(&message).unwrap();
                    network_state_event = Some(NetworkStateEvent::Message(connection, packet));
                }
                message_io::network::NetEvent::Disconnected(endpoint) => {
                    println!("Client Disconnected!");
                    self.network_state.remove_endpoint(endpoint);
                    let connection = self
                        .network_state
                        .connections
                        .get_by_right(&endpoint)
                        .unwrap()
                        .clone();
                    network_state_event = Some(NetworkStateEvent::Disconnected(connection))
                }
            }
        }
        self.network_state_publisher.publish(&mut MutEvent::new(
            &mut network_state_event.unwrap(),
            &mut self.network_state,
        ));
    }
}
