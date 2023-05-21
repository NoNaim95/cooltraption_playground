use message_io::node::{NodeListener, NodeEvent};

use crate::node_event_handler;

pub enum Signal{}



pub struct Server{
    pub listener: NodeListener<Signal>
}

impl Server {
    pub fn run(self, node_event_handler: impl FnMut(NodeEvent<Signal>)) {
        self.listener.for_each(node_event_handler)
    }
}
