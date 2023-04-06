pub struct Event<T> {
    event_handlers: Vec<Box<dyn FnMut(&T)>>,
}

impl<T> Event<T> {
    pub fn add_event_handler(&mut self, f: impl FnMut(&T) + 'static) {
        self.event_handlers.push(Box::new(f));
    }
    pub fn invoke(&mut self, payload: &T) {
        for event_handler in &mut self.event_handlers {
            event_handler(payload);
        }
    }
}

impl<T> Default for Event<T> {
    fn default() -> Self {
        Event { event_handlers: Default::default() }
    }
}

#[derive(Default)]
pub struct MutEvent<T> {
    event_handlers: Vec<Box<dyn Fn(&mut T)>>,
}

impl<T> MutEvent<T> {
    pub fn add_event_handler(&mut self, f: impl Fn(&mut T) + 'static) {
        self.event_handlers.push(Box::new(f));
    }

    pub fn invoke(&self, payload: &mut T) {
        for event_handler in &self.event_handlers {
            event_handler(payload);
        }
    }
}
