use message_io::node::{NodeHandler, NodeListener, self};


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
