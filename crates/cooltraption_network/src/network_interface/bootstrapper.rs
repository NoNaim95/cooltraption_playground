use std::{
    net::ToSocketAddrs,
    sync::{Arc, Mutex},
};

use super::*;
use log::debug;
use message_io::node::{NodeEvent, NodeListener};
use serde::{de::DeserializeOwned, Serialize};
use super::messageio_adapter::MessageIoAdapter;

pub trait NetworkInterfaceBootstrapper {
    fn connect(self, dst_addr: impl ToSocketAddrs);
    fn listen(self, bind_addr: impl ToSocketAddrs);
}

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

    pub fn concurrent_network_interface(&self) -> ConcurrentNetworkInterface<P> {
        Arc::clone(&self.concurrent_network_interface)
    }

    fn handle_event_loop(mut self) {
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
}

impl<P> NetworkInterfaceBootstrapper for MessageIoInterfaceBootstrapper<P>
where
    P: Serialize + DeserializeOwned + 'static,
{
    fn connect(self, dst_addr: impl ToSocketAddrs) {
        debug!("Connecting");
        self.message_io_adapter
            .lock()
            .unwrap()
            .node_handler()
            .network()
            .connect(
                message_io::network::Transport::FramedTcp,
                dst_addr.to_socket_addrs().unwrap().next().unwrap(),
            )
            .expect("localhost to allow outgoing connections");

        self.handle_event_loop();
    }

    fn listen(self, bind_addr: impl ToSocketAddrs) {
        self.message_io_adapter
            .lock()
            .unwrap()
            .node_handler()
            .network()
            .listen(message_io::network::Transport::FramedTcp, bind_addr)
            .unwrap();
        self.handle_event_loop();
    }
}
