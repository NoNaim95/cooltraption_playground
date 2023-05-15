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
