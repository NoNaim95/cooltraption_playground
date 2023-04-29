use crate::world_renderer::camera::controls::ButtonMap;
pub use winit::event::VirtualKeyCode;

const KEY_COUNT: usize = 163;

pub struct KeyboardState {
    keys: [bool; KEY_COUNT],
}

impl ButtonMap for KeyboardState {
    type Button = VirtualKeyCode;

    fn set_btn(&mut self, button: &Self::Button, is_down: bool) {
        self.keys[*button as usize] = is_down;
    }

    fn is_down(&self, vk_code: &VirtualKeyCode) -> bool {
        self.keys[*vk_code as usize]
    }

    fn is_up(&self, vk_code: &VirtualKeyCode) -> bool {
        !self.is_down(vk_code)
    }
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            keys: [false; KEY_COUNT],
        }
    }
}
