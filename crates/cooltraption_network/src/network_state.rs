use std::sync::{Arc, Mutex, MutexGuard};

use crate::connection::Connection;
use crate::packets::Packet;
use bimap::BiMap;

use message_io::{
    network::Endpoint,
    node::{NodeEvent, NodeHandler, NodeListener},
};

pub enum Signal {}

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

    pub fn send_packet(&self, packet: Packet<()>, connection: &Connection) {
        let endpoint = self.connections.get_by_left(connection).unwrap();
        self.node_handler.network().send(
            *endpoint,
            serde_yaml::to_string(&packet).unwrap().as_bytes(),
        );
    }

    pub fn connections(&self) -> Vec<&Connection> {
        self.connections.left_values().collect()
    }

    pub fn disconnect(&mut self, id: Connection) {
        let resource_id = self.connections.get_by_left(&id).unwrap().resource_id();
        self.node_handler.network().remove(resource_id);
        self.connections.remove_by_left(&id);
    }

    pub fn stop_listener(&mut self) {
        self.node_handler.stop();
    }

    fn add_endpoint(&mut self, endpoint: Endpoint) {
        self.connections
            .insert(Connection::new(endpoint.addr()), endpoint);
    }

    fn remove_endpoint(&mut self, endpoint: &Endpoint) {
        self.connections.remove_by_right(endpoint);
    }

    fn apply_node_event(&mut self, message: &NodeEvent<'_, Signal>) -> NetworkStateEvent {
        if let NodeEvent::Network(net_event) = message {
            let network_state_event: NetworkStateEvent = match net_event {
                message_io::network::NetEvent::Connected(endpoint, established) => {
                    if !established {
                        panic!("connection failed");
                    }
                    println!("Connected to Server!");
                    self.add_endpoint(*endpoint);
                    NetworkStateEvent::Connected(
                        self.connections.get_by_right(endpoint).unwrap().clone(),
                    )
                }
                message_io::network::NetEvent::Accepted(endpoint, _) => {
                    println!("Client Connected!");
                    self.add_endpoint(*endpoint);
                    NetworkStateEvent::Accepted(
                        self.connections.get_by_right(endpoint).unwrap().clone(),
                    )
                }
                message_io::network::NetEvent::Message(endpoint, message) => {
                    println!("Message received!");
                    let connection = self.connections.get_by_right(endpoint).unwrap().clone();
                    let packet = serde_yaml::from_slice::<Packet<()>>(message).unwrap();
                    NetworkStateEvent::Message(connection, packet)
                }
                message_io::network::NetEvent::Disconnected(endpoint) => {
                    println!("Client Disconnected!");
                    let connection = self.connections.get_by_right(endpoint).unwrap().clone();
                    self.remove_endpoint(endpoint);
                    NetworkStateEvent::Disconnected(connection)
                }
            };
            network_state_event
        } else {
            unimplemented!();
        }
    }
}
pub type ConcurrentNetworkState = Arc<Mutex<NetworkStateImpl>>;
pub type NetworkStateEventHandler =
    Box<dyn FnMut(&NetworkStateEvent, &mut MutexGuard<NetworkStateImpl>) + Send>;

pub enum NetworkStateEvent {
    Connected(Connection),
    Accepted(Connection),
    Disconnected(Connection),
    Message(Connection, Packet<()>),
}

pub struct NodeEventHandler {
    pub network_state: ConcurrentNetworkState,
    pub network_state_publisher: Vec<NetworkStateEventHandler>,
    pub node_listener: NodeListener<Signal>,
}

impl NodeEventHandler {
    pub fn new(
        network_state: ConcurrentNetworkState,
        network_state_publisher: Vec<NetworkStateEventHandler>,
        node_listener: NodeListener<Signal>,
    ) -> Self {
        Self {
            network_state,
            network_state_publisher,
            node_listener,
        }
    }

    pub fn handle_event_loop(mut self) {
        self.node_listener
            .for_each(move |event: NodeEvent<'_, Signal>| {
                let mut network_state_lock = self.network_state.lock().unwrap();
                let network_state_event = network_state_lock.apply_node_event(&event);
                for f in self.network_state_publisher.iter_mut() {
                    f(&network_state_event, &mut network_state_lock);
                }
            });
    }

    pub fn concurrent_network_state(&self) -> ConcurrentNetworkState {
        Arc::clone(&self.network_state)
    }

    pub fn node_handler(&self) -> NodeHandler<Signal> {
        self.network_state.lock().unwrap().node_handler.clone()
    }
}
