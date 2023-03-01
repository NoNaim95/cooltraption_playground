pub(crate) use render_frame::RenderFrame;

pub mod gui;
pub mod instance_renderer;
pub mod render_frame;
pub mod vertex;

pub trait Renderer {
    fn render(&mut self, render_frame: &mut RenderFrame);
}
