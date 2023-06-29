use std::{
    net::ToSocketAddrs,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use message_io::node;

use crate::network_state::{
    ConcurrentNetworkState, NetworkStateEventHandler, NetworkStateImpl, NodeEventHandler, Signal,
};

pub fn connect(
    server: impl ToSocketAddrs,
    network_state_event_handlers: Vec<NetworkStateEventHandler>,
) -> (JoinHandle<()>, ConcurrentNetworkState) {
    let (handler, listener) = node::split::<Signal>();
    handler
        .network()
        .connect(
            message_io::network::Transport::FramedTcp,
            server.to_socket_addrs().unwrap().next().unwrap(),
        )
        .expect("localhost to allow outgoing connections");

    let network_state = Arc::new(Mutex::new(NetworkStateImpl::new(handler)));
    let mut node_event_handler =
        NodeEventHandler::new(Arc::clone(&network_state), network_state_event_handlers);

    let handle = std::thread::spawn(move || {
        listener.for_each(|node_event| node_event_handler.handle_node_event(node_event));
    });

    (handle, network_state)
}

pub fn listen(
    addr: impl ToSocketAddrs,
    network_state_event_handlers: Vec<NetworkStateEventHandler>,
) -> (JoinHandle<()>, ConcurrentNetworkState) {
    let (handler, listener) = node::split::<Signal>();

    handler
        .network()
        .listen(message_io::network::Transport::FramedTcp, addr)
        .unwrap();
    let network_state = Arc::new(Mutex::new(NetworkStateImpl::new(handler)));
    let mut node_event_handler =
        NodeEventHandler::new(Arc::clone(&network_state), network_state_event_handlers);

    let handle = std::thread::spawn(move || {
        listener.for_each(|node_event| node_event_handler.handle_node_event(node_event));
    });
    (handle, network_state)
}
