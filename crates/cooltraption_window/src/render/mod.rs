pub(crate) use render_frame::RenderFrame;

pub use crate::window::controls::CameraControls;
pub use crate::window::keyboard_state::{KeyboardState, VirtualKeyCode};

pub mod gui;
pub mod instance_renderer;
pub mod render_frame;
pub mod vertex;

pub trait Renderer {
    fn render(&mut self, render_frame: &mut RenderFrame);
}
