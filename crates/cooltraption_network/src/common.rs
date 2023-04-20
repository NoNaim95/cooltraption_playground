use message_io::node::{NodeHandler, NodeListener, self, NodeEvent};
use cooltraption_common::events::EventPublisher;

pub struct NetworkingEngine {
    pub handler: NodeHandler<()>,
    pub listener: NodeListener<()>,
}

impl NetworkingEngine {
}

impl Default for NetworkingEngine {
    fn default() -> Self {
        let (handler, listener) = node::split();
        Self { handler, listener }
    }
}
