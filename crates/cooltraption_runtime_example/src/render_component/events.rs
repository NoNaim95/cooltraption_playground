use cooltraption_common::events::EventFamily;

use super::controller::CameraMovedEvent;

pub struct Event<'a, P, C = ()> {
    payload: &'a P,
    context: &'a C,
}

impl<'a, P, C> Event<'a, P, C> {
    pub fn new(payload: &'a P, context: &'a C) -> Self {
        Self { payload, context }
    }
    pub fn payload(&self) -> &P {
        &self.payload
    }
    pub fn context(&self) -> &C {
        &self.context
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
        &mut self.payload
    }
    pub fn mut_context(&mut self) -> &mut C {
        &mut self.context
    }
}

impl EventFamily for Event<'_, CameraMovedEvent> {
    type Event<'e> = Event<'e, CameraMovedEvent>;
}
