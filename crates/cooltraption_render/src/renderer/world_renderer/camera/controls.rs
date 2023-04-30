use cgmath::{EuclideanSpace, Point2};

pub trait CameraController {
    fn get_view(&self) -> Option<CameraView>;
}

#[derive(Clone, Copy, Debug)]
pub struct CameraView {
    pub position: Point2<f32>,
    pub zoom: f32,
}

impl Default for CameraView {
    fn default() -> Self {
        Self {
            position: Point2::origin(),
            zoom: 1.0,
        }
    }
}
