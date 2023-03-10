use crate::window::event_handler::EventHandler;
use egui::Context;

pub enum UiState {
    KeepOpen,
    Close,
}

pub trait GuiWindow: EventHandler {
    fn show(&mut self, context: &Context) -> UiState;
}
