use winit::window::Window;

pub(crate) use render_frame::RenderFrame;

use crate::WgpuState;

pub mod gui;
pub mod instance_renderer;
pub mod render_event_handler;
pub mod render_frame;
pub mod vertex;

pub trait Renderer {
    fn init(&mut self, window: &Window, wgpu_state: &WgpuState);
    fn render(&mut self, render_frame: &mut RenderFrame);
}
