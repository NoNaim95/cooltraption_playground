use std::cell::RefCell;
use std::rc::Rc;

pub(crate) use render_frame::RenderFrame;

use crate::Context;

pub mod gui;
pub mod instance_renderer;
pub mod render_event_handler;
pub mod render_frame;
pub mod vertex;

pub trait Renderer {
    fn render(&mut self, render_frame: &mut RenderFrame);
}

pub trait RendererInitializer {
    fn init(self: Box<Self>, context: &mut Context) -> Rc<RefCell<dyn Renderer>>;
}
