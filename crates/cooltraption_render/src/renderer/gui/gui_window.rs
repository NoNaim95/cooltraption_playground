use crate::gui::WindowId;
use cooltraption_window::events::EventHandler;
use cooltraption_window::window::{WindowContext, WinitEvent};
use egui::Context;

pub trait GuiWindow: for<'a, 'b, 'c> EventHandler<WinitEvent<'a, 'b>, WindowContext<'c>> {
    fn show(&mut self, context: &Context) -> bool;
    fn id(&self) -> WindowId;
}
