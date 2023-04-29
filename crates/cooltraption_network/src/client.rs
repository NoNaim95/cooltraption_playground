use std::{net::SocketAddrV4, time::Duration};

use message_io::{node::{self, NodeHandler, StoredNodeEvent, NodeTask}, events::EventReceiver, network::Endpoint};

use crate::server::Signal;

pub struct Client {}
impl Client {
    pub fn connect(server: SocketAddrV4, timeout: Duration) -> Option<(NodeHandler<Signal>, EventReceiver<StoredNodeEvent<Signal>>, NodeTask, Endpoint)> {
        let (handler, listener) = node::split();
        let (server, _) = handler
            .network()
            .connect(message_io::network::Transport::FramedTcp, server)
            .expect("localhost to allow outgoing connections");
        let (node_task, mut event_receiver) = listener.enqueue();
        let event = event_receiver.receive_timeout(timeout)?;
        if let node::StoredNetEvent::Connected(endpoint, success) = event.network(){
            if success && (endpoint == server) {
                return Some((handler, event_receiver, node_task, server));
            }
        }
        handler.stop();
        None
    }
}
