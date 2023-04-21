use crate::window::event_handler::EventHandler;
use egui::Context;

pub trait GuiWindow: EventHandler {
    fn show(&mut self, context: &Context);
    fn id(&self) -> &'static str;
}
