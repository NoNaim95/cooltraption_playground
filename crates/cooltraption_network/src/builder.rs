use std::sync::Arc;
use std::sync::Mutex;

use message_io::node;
use message_io::node::NodeListener;

use crate::network_state::ConcurrentNetworkState;
use crate::network_state::NetworkStateEventHandler;
use crate::network_state::NetworkStateImpl;
use crate::network_state::NodeEventHandler;
use crate::network_state::Signal;

pub struct NodeEventHandlerBuilder {
    pub network_state: ConcurrentNetworkState,
    pub network_state_publisher: Vec<NetworkStateEventHandler>,
    pub node_listener: NodeListener<Signal>,
}

impl Default for NodeEventHandlerBuilder {
    fn default() -> Self {
        let (node_handler, node_listener) = node::split::<Signal>();
        Self {
            network_state: Arc::new(Mutex::new(NetworkStateImpl::new(node_handler))),
            network_state_publisher: vec![],
            node_listener,
        }
    }
}

impl NodeEventHandlerBuilder {
    pub fn add_network_state_event_handler(&mut self, handler: NetworkStateEventHandler) {
        self.network_state_publisher.push(handler);
    }

    pub fn build(self) -> NodeEventHandler {
        NodeEventHandler::new(
            self.network_state,
            self.network_state_publisher,
            self.node_listener,
        )
    }
}
