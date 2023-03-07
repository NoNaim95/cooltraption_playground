pub use keyboard_state::KeyboardState;
pub use mouse_state::MouseState;
use std::ops::{AddAssign, SubAssign};

mod keyboard_state;
mod mouse_state;

pub trait ButtonState: Default + AddAssign<Self::Button> + SubAssign<Self::Button> {
    type Button;

    fn is_down(&self, button: Self::Button) -> bool;
    fn is_up(&self, button: Self::Button) -> bool;
}
