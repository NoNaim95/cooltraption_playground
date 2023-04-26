use crate::window::{WindowContext, WinitEvent};
use cgmath::Vector2;
use cooltraption_common::events::EventHandler;
use num_traits::Zero;

pub use self::keyboard_state::*;
pub use self::mouse_state::*;

mod keyboard_state;
mod mouse_state;

pub trait ButtonMap: Default {
    type Button;

    fn set_btn(&mut self, button: &Self::Button, is_down: bool);
    fn is_down(&self, button: &Self::Button) -> bool;
    fn is_up(&self, button: &Self::Button) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct CameraControls {
    pub move_vec: Vector2<f32>,
    pub zoom: f32,
}

impl Default for CameraControls {
    fn default() -> Self {
        Self {
            move_vec: Vector2::zero(),
            zoom: 1.0,
        }
    }
}

pub trait CameraController:
    for<'s, 'a, 'b, 'c> EventHandler<'s, WinitEvent<'a, 'b>, WindowContext<'c>>
{
}
