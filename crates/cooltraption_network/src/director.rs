use std::sync::MutexGuard;

use crate::{
    builder::NodeEventHandlerBuilder,
    network_state::{NetworkStateEvent, NetworkStateImpl, NodeEventHandler},
};

pub fn networker() -> NodeEventHandler {
    let mut builder = NodeEventHandlerBuilder::default();
    let handler = |_event: &NetworkStateEvent, _locked_state: &mut MutexGuard<NetworkStateImpl>| {};

    builder.add_network_state_event_handler(Box::new(handler));
    builder.build()
}
