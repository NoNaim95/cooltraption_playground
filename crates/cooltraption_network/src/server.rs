use cooltraption_common::events::MutEventPublisher;

pub use message_io::network::{Endpoint, NetEvent, Transport};
pub use message_io::node::{self, NodeHandler, NodeListener, StoredNodeEvent};
pub struct ServerNetworkingEngine {}

impl ServerNetworkingEngine {
    pub fn run(
        &mut self,
        port: u16,
        mut node_event_publisher: MutEventPublisher<(StoredNodeEvent<Signal>, Context)>,
    ) {
        let (handler, listener) = node::split();
        handler
            .network()
            .listen(Transport::FramedTcp, format!("0.0.0.0:{}", port))
            .expect("The port to be free");

        let (_task, mut events) = listener.enqueue();

        loop {
            let event = events.receive();
            let context = Context {
                node_handler: handler.clone(),
            };

            node_event_publisher.publish(&mut (event, context));
        }
    }
}

#[derive(Clone)]
pub struct Context {
    pub node_handler: NodeHandler<Signal>,
}

#[derive(Default, Debug)]
pub struct MessageStorage {
    pub sent_messages: Vec<(Endpoint, String)>,
}

#[derive(Debug, Clone)]
pub enum Signal {
    DisconnectClient(Endpoint),
}
