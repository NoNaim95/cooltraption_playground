use crate::controls::ButtonMap;
use cgmath::{EuclideanSpace, Point2};
use cooltraption_window::window::winit::event::MouseButton;

const BUTTON_COUNT: usize = 163;

#[derive(Clone)]
pub struct MouseState {
    buttons: [bool; BUTTON_COUNT],
    scroll: f32,
    pos: Point2<f32>,
}

impl MouseState {
    pub fn reset(&mut self) {
        self.scroll = 0.0;
    }

    pub fn add_scroll(&mut self, delta: f32) {
        self.scroll += delta;
    }

    pub fn scroll(&self) -> f32 {
        self.scroll
    }

    pub fn set_pos(&mut self, pos: Point2<f32>) {
        self.pos = pos;
    }

    pub fn pos(&self) -> Point2<f32> {
        self.pos
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
            pos: Point2::origin(),
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
