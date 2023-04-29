use crate::events::EventHandler;
use crate::window::{WindowContext, WinitEvent};
use egui::Context;

pub trait GuiWindow: for<'a, 'b, 'c> EventHandler<WinitEvent<'a, 'b>, WindowContext<'c>> {
    fn show(&mut self, context: &Context);
    fn id(&self) -> &'static str;
}
