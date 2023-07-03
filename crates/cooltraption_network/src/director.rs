use std::sync::MutexGuard;

use crate::{
    builder::NodeEventHandlerBuilder,
    network_state::{NetworkStateEvent, NetworkStateImpl, NodeEventHandler},
};

pub fn networker<T>() -> NodeEventHandler<T> {
    let mut builder = NodeEventHandlerBuilder::default();
    let handler = |_event: &NetworkStateEvent<T>, _locked_state: &mut MutexGuard<NetworkStateImpl<T>>| {};

    builder.add_network_state_event_handler(Box::new(handler));
    builder.build()
}
