use std::net::ToSocketAddrs;

use log::debug;
use serde::de::DeserializeOwned;

use crate::network_state::NodeEventHandler;

pub fn connect<T>(server: impl ToSocketAddrs, node_event_handler: NodeEventHandler<T>)
where
    T: DeserializeOwned,
{
    debug!("Connecting");
    node_event_handler
        .node_handler()
        .network()
        .connect(
            message_io::network::Transport::FramedTcp,
            server.to_socket_addrs().unwrap().next().unwrap(),
        )
        .expect("localhost to allow outgoing connections");

    node_event_handler.handle_event_loop();
}

pub fn listen<T>(addr: impl ToSocketAddrs, node_event_handler: NodeEventHandler<T>)
where
    T: DeserializeOwned,
{
    node_event_handler
        .node_handler()
        .network()
        .listen(message_io::network::Transport::FramedTcp, addr)
        .unwrap();
    node_event_handler.handle_event_loop();
}
