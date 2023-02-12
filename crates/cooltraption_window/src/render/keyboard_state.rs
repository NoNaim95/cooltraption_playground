use std::ops::{AddAssign, SubAssign};

use winit::event::VirtualKeyCode;

const KEY_COUNT: usize = 163;

pub struct KeyboardState {
    keys: [bool; KEY_COUNT],
}

impl KeyboardState {
    pub fn is_down(&self, vk_code: VirtualKeyCode) -> bool {
        self.keys[vk_code as usize]
    }

    pub fn is_up(&self, vk_code: VirtualKeyCode) -> bool {
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

impl AddAssign<VirtualKeyCode> for KeyboardState {
    fn add_assign(&mut self, rhs: VirtualKeyCode) {
        let index = rhs as usize;
        self.keys[index] = true;
    }
}

impl SubAssign<VirtualKeyCode> for KeyboardState {
    fn sub_assign(&mut self, rhs: VirtualKeyCode) {
        let index = rhs as usize;
        self.keys[index] = false;
    }
}
