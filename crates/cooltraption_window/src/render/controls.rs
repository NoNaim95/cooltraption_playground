use cgmath::Vector2;
use num_traits::Zero;

#[derive(Clone)]
pub struct CameraControls {
    pub move_vec: Vector2<f32>,
    pub zoom: f32,
}

impl Default for CameraControls {
    fn default() -> Self {
        Self {
            move_vec: Vector2::zero(),
            zoom: 1.0,
        }
    }
}
