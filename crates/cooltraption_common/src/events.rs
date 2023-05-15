pub trait EventFamily {
    type Event<'e>;
}

use smart_default::SmartDefault;
#[derive(SmartDefault)]
pub struct EventPublisher<'a, T: EventFamily> {
    event_handlers: Vec<Box<dyn for<'e> EventHandler<T::Event<'e>> + 'a>>,
}

impl<'a, T: EventFamily> EventPublisher<'a, T> {
    pub fn add_event_handler(
        &mut self,
        event_handler: impl for<'e> EventHandler<T::Event<'e>> + 'a,
    ) {
        self.event_handlers.push(Box::new(event_handler));
    }

    pub fn publish(&mut self, payload: &T::Event<'_>) {
        for event_handler in &mut self.event_handlers {
            event_handler.handle_event(payload);
        }
    }
}

pub trait EventHandler<T> {
    fn handle_event(&mut self, event: &T);
}

impl<E, F> EventHandler<E> for F
where
    F: for<'a> FnMut(&'a E),
{
    fn handle_event(&mut self, event: &E) {
        self(event)
    }
}

#[derive(SmartDefault)]
pub struct MutEventPublisher<'a, T: EventFamily> {
    event_handlers: Vec<Box<dyn for<'e> MutEventHandler<T::Event<'e>> + 'a>>,
}


impl<'a, T: EventFamily> MutEventPublisher<'a, T> {
    pub fn add_event_handler(
        &mut self,
        event_handler: impl for<'e> MutEventHandler<T::Event<'e>> + 'a,
    ) {
        self.event_handlers.push(Box::new(event_handler));
    }

    pub fn publish(&mut self, payload: &mut T::Event<'_>) {
        for event_handler in &mut self.event_handlers {
            event_handler.handle_event(payload);
        }
    }
}

pub trait MutEventHandler<T> {
    fn handle_event(&mut self, event: &mut T);
}

impl<E, F> MutEventHandler<E> for F
where
    F: for<'a> FnMut(&'a mut E),
{
    fn handle_event(&mut self, event: &mut E) {
        self(event)
    }
}
