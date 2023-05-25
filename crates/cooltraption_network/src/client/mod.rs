use crate::{events::Event, server::Signal};
use cooltraption_common::events::EventPublisher;
use message_io::{
    network::Endpoint,
    node::{self, NodeEvent, NodeHandler, NodeListener},
};
use std::net::SocketAddrV4;

#[derive(Default)]
pub struct Client {}
impl Client {
    pub fn connect(server: SocketAddrV4) -> (NodeHandler<Signal>, NodeListener<Signal>, Endpoint) {
        let (handler, listener) = node::split::<Signal>();
        let (server, _) = handler
            .network()
            .connect(message_io::network::Transport::FramedTcp, server)
            .expect("localhost to allow outgoing connections");
        (handler, listener, server)
    }
}

fn run_listener<'a>(
    listener: NodeListener<Signal>,
    mut publisher: EventPublisher<'a, Event<'a, NodeEvent<'a, Signal>>>,
) {
    let f = move |event: NodeEvent<Signal>| {
        publisher.publish(&Event::new(&event, &()));
    };

    listener.for_each(f);
}
