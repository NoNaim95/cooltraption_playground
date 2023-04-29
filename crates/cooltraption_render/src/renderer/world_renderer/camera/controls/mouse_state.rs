use crate::world_renderer::camera::controls::ButtonMap;
use cgmath::Vector2;
use num_traits::Zero;
use std::time::Duration;
use winit::event::MouseButton;

const BUTTON_COUNT: usize = 163;

#[derive(Clone)]
pub struct MouseState {
    buttons: [bool; BUTTON_COUNT],
    scroll_smoothness: f32,
    scroll: f32,
    pos: Vector2<f64>,
    pos_delta: Vector2<f64>,
}

impl MouseState {
    pub fn reset(&mut self, delta_time: &Duration) {
        self.scroll -= self.scroll(delta_time);
        self.pos_delta = Vector2::zero();
    }

    pub fn add_scroll(&mut self, delta: f32) {
        self.scroll += delta;
    }

    pub fn scroll(&self, delta_time: &Duration) -> f32 {
        (self.scroll * (1.0 / self.scroll_smoothness) * delta_time.as_secs_f32())
            .max(-(self.scroll.abs()))
            .min(self.scroll.abs())
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
            scroll_smoothness: 0.2,
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