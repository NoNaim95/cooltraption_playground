pub use renderer::gui;
pub use renderer::world_renderer;

pub mod camera;
pub mod renderer;
pub mod window;

pub trait Context {}
pub trait Event {}

pub trait EventProxy<'s, E: Event, C: Context, H> {
    fn register_event_handler(&'s mut self, handler: H);
    fn send_event(&'s mut self, event: &mut E);
}

pub trait EventHandler<'s, E: Event, C: Context> {
    fn handle_event(&'s mut self, event: &mut E, context: &mut C);
}
