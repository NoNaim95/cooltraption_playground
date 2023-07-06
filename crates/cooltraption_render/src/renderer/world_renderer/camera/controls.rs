use cgmath::{EuclideanSpace, InnerSpace, Point2, Vector2};

pub trait CameraController {
    fn get_view(&self) -> Option<CameraView>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraView {
    pub position: Point2<f32>,
    pub zoom: f32,
}

impl CameraView {
    pub fn world_pos(&self, window_pos: Point2<f32>, window_size: Vector2<f32>) -> Point2<f32> {
        let mag = window_size.magnitude();
        let offset = Point2::new(
            (window_pos.x - window_size.x / 2.0) / mag,
            (window_size.y / 2.0 - window_pos.y) / mag,
        );
        let scaled_offset = offset / self.zoom;
        Point2::new(
            self.position.x + scaled_offset.x,
            self.position.y + scaled_offset.y,
        )
    }
}

impl Default for CameraView {
    fn default() -> Self {
        Self {
            position: Point2::origin(),
            zoom: 1.0,
        }
    }
}
