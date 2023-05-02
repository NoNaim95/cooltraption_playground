use super::ButtonMap;
use cgmath::{Vector2, Zero};
use cooltraption_render::window::winit::event::MouseButton;

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
}

impl ButtonMap for MouseState {
    type Button = MouseButton;

    fn set_btn(&mut self, button: &Self::Button, is_down: bool) {
        self.buttons[as_usize(button)] = is_down;
    }

    fn is_down(&self, button: &MouseButton) -> bool {
        self.buttons[as_usize(button)]
    }

    fn is_up(&self, button: &MouseButton) -> bool {
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

fn as_usize(btn: &MouseButton) -> usize {
    match btn {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(b) => *b as usize,
    }
}
