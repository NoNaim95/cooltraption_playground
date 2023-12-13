use std::{
    marker::PhantomData,
    net::ToSocketAddrs,
    sync::{atomic::AtomicPtr, Arc, Mutex, MutexGuard},
};

use crate::connection::Connection;
use crate::packets::Packet;
use bimap::BiMap;

use log::debug;
use message_io::{
    network::Endpoint,
    node::{NodeEvent, NodeHandler, NodeListener},
};
use serde::{de::DeserializeOwned, Serialize};

pub enum Signal {}

pub trait NetworkInterface<P> {
    fn send_packet(&self, packet: Packet<P>, connection: &Connection);
    fn connections(&self) -> Vec<Connection>;
    fn disconnect(&mut self, id: &Connection);
    fn stop_listener(&mut self);
}

pub struct NetworkInterfaceWrapper<T, P>
where
    T: NetworkInterface<P>,
{
    nw_interface: Arc<Mutex<T>>,
    _phantom: PhantomData<T>,
    _phantom_p: PhantomData<fn() -> P>,
}

impl<T, P> NetworkInterface<P> for NetworkInterfaceWrapper<T, P>
where
    P: Serialize + DeserializeOwned,
    T: NetworkInterface<P>,
{
    fn send_packet(&self, packet: Packet<P>, connection: &Connection) {
        self.nw_interface
            .lock()
            .unwrap()
            .send_packet(packet, connection)
    }

    fn connections(&self) -> Vec<Connection> {
        self.nw_interface.lock().unwrap().connections()
    }

    fn disconnect(&mut self, id: &Connection) {
        self.nw_interface.lock().unwrap().disconnect(id)
    }

    fn stop_listener(&mut self) {
        self.nw_interface.lock().unwrap().stop_listener()
    }
}
impl<T, P> NetworkInterfaceWrapper<T, P>
where
    P: Serialize + DeserializeOwned,
    T: NetworkInterface<P>,
{
    pub fn new(nw_interface: T) -> Self {
        Self {
            nw_interface: Arc::new(Mutex::new(nw_interface)),
            _phantom: PhantomData,
            _phantom_p: PhantomData,
        }
    }

    pub fn network_interface(&self) -> Arc<Mutex<T>> {
        Arc::clone(&self.nw_interface)
    }
}

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

    fn node_handler(&self) -> NodeHandler<Signal> {
        self.node_handler.clone()
    }

    fn apply_node_event(&mut self, message: &NodeEvent<'_, Signal>) -> NetworkInterfaceEvent<T>
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
pub type ConcurrentNetworkState<T> = Arc<Mutex<MessageIoAdapter<T>>>;
pub type NetworkStateEventHandler<T> =
    Box<dyn FnMut(&NetworkInterfaceEvent<T>, &mut MutexGuard<MessageIoAdapter<T>>) + Send>;

pub type ConcurrentNetworkInterface<P> = Arc<Mutex<dyn NetworkInterface<P> + 'static + Send>>;
pub type NetworkInterfaceEventHandler<P> = Box<
    dyn FnMut(&NetworkInterfaceEvent<P>, &mut ConcurrentNetworkInterface<P>) + Send + 'static,
>;

pub enum NetworkInterfaceEvent<T> {
    Connected(Connection),
    Accepted(Connection),
    Disconnected(Connection),
    Message(Connection, Packet<T>),
}

pub trait NetworkInterfaceBootstrapper {
    fn start_listener();
}

/// Responsible for passing each Message-io message to the Networkstate
/// and publishing the NetworkState after that
pub struct MessageIoInterfaceBootstrapper<P>
where
    P: Serialize + DeserializeOwned,
{
    concurrent_network_interface: ConcurrentNetworkInterface<P>,
    message_io_adapter: Arc<Mutex<MessageIoAdapter<P>>>,
    network_interface_publisher: Vec<NetworkInterfaceEventHandler<P>>,
    node_listener: NodeListener<Signal>,
}

impl<P> MessageIoInterfaceBootstrapper<P>
where
    P: Serialize + DeserializeOwned + 'static,
{
    pub fn new(
        interface_wrapper: NetworkInterfaceWrapper<MessageIoAdapter<P>, P>,
        network_interface_publisher: Vec<NetworkInterfaceEventHandler<P>>,
        node_listener: NodeListener<Signal>,
    ) -> Self {
        Self {
            message_io_adapter: interface_wrapper.network_interface(),
            concurrent_network_interface: Arc::new(Mutex::new(interface_wrapper)),
            network_interface_publisher,
            node_listener,
        }
    }

    pub fn handle_event_loop(mut self)
    where
        P: DeserializeOwned + 'static,
    {
        self.node_listener
            .for_each(move |event: NodeEvent<'_, Signal>| {
                let mut network_interface_clone = self.concurrent_network_interface.clone();
                let network_interface_event = self
                    .message_io_adapter
                    .lock()
                    .unwrap()
                    .apply_node_event(&event);
                for f in self.network_interface_publisher.iter_mut() {
                    f(&network_interface_event, &mut network_interface_clone);
                }
            });
    }

    pub fn concurrent_network_interface(&self) -> ConcurrentNetworkInterface<P> {
        Arc::clone(&self.concurrent_network_interface)
    }

    pub fn connect(self, server_addr: impl ToSocketAddrs) {
        debug!("Connecting");
        self.message_io_adapter
            .lock()
            .unwrap()
            .node_handler()
            .network()
            .connect(
                message_io::network::Transport::FramedTcp,
                server_addr.to_socket_addrs().unwrap().next().unwrap(),
            )
            .expect("localhost to allow outgoing connections");

        self.handle_event_loop();
    }

    pub fn listen(self, addr: impl ToSocketAddrs) {
        self.message_io_adapter
            .lock()
            .unwrap()
            .node_handler()
            .network()
            .listen(message_io::network::Transport::FramedTcp, addr)
            .unwrap();
        self.handle_event_loop();
    }
}
