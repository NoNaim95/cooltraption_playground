use std::sync::Arc;
use std::sync::Mutex;

use message_io::node;
use message_io::node::NodeListener;

use crate::network_state::ConcurrentNetworkState;
use crate::network_state::NetworkStateEventHandler;
use crate::network_state::NetworkStateImpl;
use crate::network_state::NodeEventHandler;
use crate::network_state::Signal;

pub struct NodeEventHandlerBuilder<T> {
    pub network_state: ConcurrentNetworkState<T>,
    pub network_state_publisher: Vec<NetworkStateEventHandler<T>>,
    pub node_listener: NodeListener<Signal>,
}

impl<T> Default for NodeEventHandlerBuilder<T> {
    fn default() -> Self {
        let (node_handler, node_listener) = node::split::<Signal>();
        Self {
            network_state: Arc::new(Mutex::new(NetworkStateImpl::new(node_handler))),
            network_state_publisher: vec![],
            node_listener,
        }
    }
}

impl<T> NodeEventHandlerBuilder<T> {
    pub fn add_network_state_event_handler(&mut self, handler: NetworkStateEventHandler<T>) {
        self.network_state_publisher.push(handler);
    }

    pub fn build(self) -> NodeEventHandler<T> {
        NodeEventHandler::new(
            self.network_state,
            self.network_state_publisher,
            self.node_listener,
        )
    }
}
