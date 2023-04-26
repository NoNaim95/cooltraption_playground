use crate::window::{WindowContext, WindowEvent};
use crate::EventHandler;
use egui::Context;
use winit::event::Event;

pub trait GuiWindow<'s>:
    for<'a, 'b> EventHandler<'s, Event<'a, WindowEvent>, WindowContext<'b>>
{
    fn show(&mut self, context: &Context);
    fn id(&self) -> &'static str;
}
