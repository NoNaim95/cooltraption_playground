use crate::network_state::ConcurrentNetworkState;
use crate::network_state::NetworkStateEventHandler;
use crate::network_state::NodeEventHandler;

pub struct NodeEventHandlerBuilder {
    pub network_state: Option<ConcurrentNetworkState>,
    pub network_state_publisher: Vec<NetworkStateEventHandler>,
}

impl NodeEventHandlerBuilder {
    pub fn add_network_state_publisher(&mut self, handler: NetworkStateEventHandler) {
        self.network_state_publisher.push(handler);
    }

    pub fn set_network_state(&mut self, network_state: ConcurrentNetworkState) {
        self.network_state = Some(network_state);
    }

    pub fn build(self) -> NodeEventHandler {
        NodeEventHandler::new(self.network_state.unwrap(), self.network_state_publisher)
    }
}
