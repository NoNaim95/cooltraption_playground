pub use keyboard_state::KeyboardState;
pub use mouse_state::MouseState;

mod keyboard_state;
mod mouse_state;

pub trait ButtonMap: Default {
    type Button;

    fn set_btn(&mut self, button: &Self::Button, is_down: bool);
    fn is_down(&self, button: &Self::Button) -> bool;
    fn is_up(&self, button: &Self::Button) -> bool;
}
