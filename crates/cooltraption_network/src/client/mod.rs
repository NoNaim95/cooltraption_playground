use std::net::ToSocketAddrs;

use crate::network_state::NodeEventHandler;

pub fn connect(server: impl ToSocketAddrs, node_event_handler: NodeEventHandler) {
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

pub fn listen(addr: impl ToSocketAddrs, node_event_handler: NodeEventHandler) {
    node_event_handler
        .node_handler()
        .network()
        .listen(message_io::network::Transport::FramedTcp, addr)
        .unwrap();
    node_event_handler.handle_event_loop();
}
