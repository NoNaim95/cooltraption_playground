pub trait Context {}
pub trait Event {}

pub trait EventProxy<'s, E: Event, C: Context, H> {
    fn register_event_handler(&'s mut self, handler: H);
    fn send_event(&'s mut self, event: &mut E);
}

pub trait EventHandler<E: Event, C: Context> {
    fn handle_event(&mut self, event: &mut E, context: &mut C);
}
