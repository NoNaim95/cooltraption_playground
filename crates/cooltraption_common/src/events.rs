pub struct EventPublisher<'a, T> {
    event_handlers: Vec<Box<dyn EventHandler<T> + 'a>>,
}

impl<'a, T> EventPublisher<'a, T> {
    pub fn add_event_handler(&mut self, event_handler: impl EventHandler<T> + 'a) {
        self.event_handlers.push(Box::new(event_handler));
    }

    pub fn publish(&mut self, payload: &T) {
        for event_handler in &mut self.event_handlers {
            event_handler.handle_event(payload);
        }
    }
}

impl<'a, T> Default for EventPublisher<'a, T> {
    fn default() -> Self {
        Self {
            event_handlers: Default::default(),
        }
    }
}

pub struct MutEventPublisher<T> {
    event_handlers: Vec<Box<dyn MutEventHandler<T>>>,
}

impl<T> MutEventPublisher<T> {
    pub fn add_event_handler(&mut self, event_handler: impl MutEventHandler<T> + 'static) {
        self.event_handlers.push(Box::new(event_handler));
    }

    pub fn publish(&mut self, payload: &mut T) {
        for event_handler in &mut self.event_handlers {
            event_handler.handle_event(payload);
        }
    }
}

impl<T> Default for MutEventPublisher<T> {
    fn default() -> Self {
        Self {
            event_handlers: Default::default(),
        }
    }
}

pub trait EventHandler<T> {
    fn handle_event(&mut self, event: &T);
}

impl<T, F: FnMut(&T)> EventHandler<T> for F {
    fn handle_event(&mut self, event: &T) {
        self(event)
    }
}

pub trait MutEventHandler<T> {
    fn handle_event(&mut self, event: &mut T);
}

impl<T, F: FnMut(&mut T)> MutEventHandler<T> for F {
    fn handle_event(&mut self, event: &mut T) {
        self(event)
    }
}
