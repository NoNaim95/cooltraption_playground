use std::sync::{Mutex, Arc};

use cooltraption_common::events::EventFamily;
use message_io::node::NodeEvent;

use crate::{server::Signal, node_event_handler::{NetworkStateImpl, NetworkState, NetworkStateEvent}};

pub struct Event<'a, P, C = ()> {
    payload: &'a P,
    context: &'a C,
}

impl<'a, P, C> Event<'a, P, C> {
    pub fn new(payload: &'a P, context: &'a C) -> Self {
        Self { payload, context }
    }
    pub fn payload(&self) -> &P {
        self.payload
    }
    pub fn context(&self) -> &C {
        self.context
    }
}


pub struct MutEvent<'a, P, C = ()> {
    payload: &'a mut P,
    context: &'a mut C,
}

impl<'a, P, C> MutEvent<'a, P, C> {
    pub fn new(payload: &'a mut P, context: &'a mut C) -> Self {
        Self { payload, context }
    }
    pub fn mut_payload(&mut self) -> &mut P {
        self.payload
    }
    pub fn mut_context(&mut self) -> &mut C {
        self.context
    }
}

type ConcurrentNetworkState<'a> = Arc<Mutex<dyn NetworkState<'a>>>;
type BoxedNetworkState<'a> = Box<dyn NetworkState<'a>>;

impl<'a> EventFamily for Event<'a, NetworkStateEvent, ConcurrentNetworkState<'a>>{
    type Event<'e> = Event<'e, NetworkStateEvent, ConcurrentNetworkState<'e>>;
}

impl<'a> EventFamily for Event<'a, NetworkStateEvent, BoxedNetworkState<'a>>{
    type Event<'e> = Event<'e, NetworkStateEvent, BoxedNetworkState<'e>>;
}

impl<'a> EventFamily for Event<'a, NetworkStateEvent, NetworkStateImpl>{
    type Event<'e> = Event<'e, NetworkStateEvent, NetworkStateImpl>;
}

impl<'a> EventFamily for Event<'a, NodeEvent<'a, Signal>, NetworkStateImpl>{
    type Event<'e> = Event<'e, NodeEvent<'e, Signal>, NetworkStateImpl>;
}

impl<'a> EventFamily for Event<'a, NodeEvent<'a, Signal>>{
    type Event<'e> = Event<'e, NodeEvent<'e, Signal>>;
}





impl<'a> EventFamily for MutEvent<'a, NetworkStateEvent, ConcurrentNetworkState<'a>>{
    type Event<'e> = MutEvent<'e, NetworkStateEvent, ConcurrentNetworkState<'e>>;
}

impl<'a> EventFamily for MutEvent<'a, NetworkStateEvent, BoxedNetworkState<'a>>{
    type Event<'e> = MutEvent<'e, NetworkStateEvent, BoxedNetworkState<'e>>;
}

impl<'a> EventFamily for MutEvent<'a, NetworkStateEvent, NetworkStateImpl>{
    type Event<'e> = MutEvent<'e, NetworkStateEvent, NetworkStateImpl>;
}
