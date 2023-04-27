use crate::window::{WindowContext, WinitEvent};
use crate::events::EventHandler;
use egui::Context;

pub trait GuiWindow<'s>:
    for<'a, 'b, 'c> EventHandler<'s, WinitEvent<'a, 'b>, WindowContext<'c>>
{
    fn show(&mut self, context: &Context);
    fn id(&self) -> &'static str;
}
