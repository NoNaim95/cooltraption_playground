use cgmath::{Matrix4, Point3, Vector3};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub target: Point3<f32>,
    pub aspect: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(
            self.target + Vector3::unit_z(),
            self.target,
            Vector3::unit_y(),
        );
        let proj = cgmath::ortho(
            -self.aspect,
            self.aspect,
            -1.0,
            1.0,
            self.z_near,
            self.z_far,
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

    pub(crate) fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
