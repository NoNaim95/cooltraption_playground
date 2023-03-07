use cgmath::Vector2;
use num_traits::Zero;
use std::ops::{AddAssign, SubAssign};
use winit::event::MouseButton;

use crate::camera::input_device::ButtonState;
pub use winit::event::VirtualKeyCode;

const BUTTON_COUNT: usize = 163;

#[derive(Clone)]
pub struct MouseState {
    buttons: [bool; BUTTON_COUNT],
    scroll: f32,
    pos: Vector2<f64>,
    pos_delta: Vector2<f64>,
}

impl MouseState {
    pub fn reset(&mut self) {
        self.scroll = 0.0;
        self.pos_delta = Vector2::zero();
    }

    pub fn add_scroll(&mut self, delta: f32) {
        self.scroll += delta;
    }

    pub fn scroll(&self) -> f32 {
        self.scroll
    }

    pub fn set_pos(&mut self, pos: Vector2<f64>) {
        let delta = pos - self.pos;

        self.pos_delta += delta;
        self.pos = pos;
    }

    pub fn pos(&self) -> Vector2<f64> {
        self.pos
    }

    pub fn pos_delta(&self) -> Vector2<f64> {
        self.pos_delta
    }
}

impl ButtonState for MouseState {
    type Button = MouseButton;

    fn is_down(&self, button: MouseButton) -> bool {
        self.buttons[as_usize(button)]
    }

    fn is_up(&self, button: MouseButton) -> bool {
        !self.is_down(button)
    }
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            buttons: [false; BUTTON_COUNT],
            scroll: 0.0,
            pos: Vector2::zero(),
            pos_delta: Vector2::zero(),
        }
    }
}

impl AddAssign<MouseButton> for MouseState {
    fn add_assign(&mut self, rhs: MouseButton) {
        let index = as_usize(rhs);
        self.buttons[index] = true;
    }
}

impl SubAssign<MouseButton> for MouseState {
    fn sub_assign(&mut self, rhs: MouseButton) {
        let index = as_usize(rhs);
        self.buttons[index] = false;
    }
}

fn as_usize(btn: MouseButton) -> usize {
    match btn {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(b) => b as usize,
    }
}
