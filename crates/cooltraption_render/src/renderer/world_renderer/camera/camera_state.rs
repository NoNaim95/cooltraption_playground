use cgmath::{EuclideanSpace, InnerSpace, Matrix4, Point2, Point3, Vector2, Vector3};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct CameraState {
    pub pos: Point2<f32>,
    pub size: Vector2<f32>,
}

impl CameraState {
    pub fn new(size: Vector2<f32>) -> Self {
        Self {
            pos: Point2::origin(),
            size: size.normalize(),
        }
    }

    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let pos = Point3::new(self.pos.x, self.pos.y, 0.0);
        let view = Matrix4::look_at_rh(pos + Vector3::unit_z(), pos, Vector3::unit_y());
        let proj = cgmath::ortho(
            -self.size.x / 2.0,
            self.size.x / 2.0,
            -self.size.y / 2.0,
            self.size.y / 2.0,
            f32::MIN,
            f32::MAX,
        );

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub(crate) fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub(crate) fn update_view_proj(&mut self, camera: &CameraState) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
