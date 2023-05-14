const KEY_COUNT: usize = 163;
use cooltraption_window::window::winit::event::VirtualKeyCode;

#[derive(Debug, Clone)]
pub struct KeyboardState {
    keys_pressed: [bool; KEY_COUNT],
}
#[allow(dead_code)]
impl KeyboardState {
    pub fn set_button(&mut self, button: &VirtualKeyCode, is_down: bool) {
        self.keys_pressed[*button as usize] = is_down;
    }

    pub fn is_down(&self, vk_code: &VirtualKeyCode) -> bool {
        self.keys_pressed[*vk_code as usize]
    }

    pub fn is_up(&self, vk_code: &VirtualKeyCode) -> bool {
        !self.is_down(vk_code)
    }
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            keys_pressed: [false; KEY_COUNT],
        }
    }
}
