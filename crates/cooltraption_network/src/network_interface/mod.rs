use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::connection::Connection;
use crate::packets::Packet;

use serde::{de::DeserializeOwned, Serialize};

pub mod bootstrapper;
pub mod messageio_adapter;

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

pub type ConcurrentNetworkInterface<P> = Arc<Mutex<dyn NetworkInterface<P> + 'static + Send>>;
pub type NetworkInterfaceEventHandler<P> =
    Box<dyn FnMut(&NetworkInterfaceEvent<P>, &mut ConcurrentNetworkInterface<P>) + Send + 'static>;

pub enum NetworkInterfaceEvent<T> {
    Connected(Connection),
    Accepted(Connection),
    Disconnected(Connection),
    Message(Connection, Packet<T>),
}
