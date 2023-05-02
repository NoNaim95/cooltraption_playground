use crate::gui::WidgetId;
use cooltraption_window::events::EventHandler;
use cooltraption_window::window::{WindowContext, WinitEvent};
use egui::Context;

pub trait Widget: for<'a, 'b, 'c> EventHandler<WinitEvent<'a, 'b>, WindowContext<'c>> {
    fn show(&mut self, context: &Context) -> bool;
    fn id(&self) -> WidgetId;
}
