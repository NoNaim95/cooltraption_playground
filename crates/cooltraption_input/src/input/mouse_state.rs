use cooltraption_window::window::winit::dpi::PhysicalPosition;
use cooltraption_window::window::winit::event::MouseButton;
use std::collections::HashSet;

#[derive(Default, Debug, Clone)]
pub struct MouseState {
    mouse_position: PhysicalPosition<f64>,
    clicked_buttons: HashSet<MouseButton>,
}
#[allow(dead_code)]
impl MouseState {
    pub fn set_mouse_position(&mut self, pos: PhysicalPosition<f64>) {
        self.mouse_position = pos;
    }

    pub fn mouse_position(&self) -> PhysicalPosition<f64> {
        self.mouse_position
    }

    pub fn set_button(&mut self, button: &MouseButton, is_down: bool) {
        if is_down {
            self.clicked_buttons.insert(*button);
        } else {
            self.clicked_buttons.remove(button);
        }
    }

    pub fn is_down(&self, button: &MouseButton) -> bool {
        self.clicked_buttons.contains(button)
    }

    pub fn is_up(&self, button: &MouseButton) -> bool {
        !self.clicked_buttons.contains(button)
    }
}
