use std::cell::RefCell;
use std::rc::Rc;

use crate::window::event_handler::Context;
use render_frame::RenderFrame;

pub mod gui;
pub mod world_renderer;
pub mod render_event_handler;
pub mod render_frame;
pub mod vertex;

pub type SharedRenderer = Rc<RefCell<dyn Renderer>>;
pub type BoxedRendererInitializer = Box<dyn RendererInitializer>;

pub trait Renderer {
    fn render(&mut self, render_frame: &mut RenderFrame);
}

pub trait RendererInitializer {
    fn init(self: Box<Self>, context: &mut Context) -> SharedRenderer;
}
