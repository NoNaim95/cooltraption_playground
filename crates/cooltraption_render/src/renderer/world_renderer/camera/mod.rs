use cgmath::{InnerSpace, Vector2};
use wgpu::util::DeviceExt;
use wgpu::*;
use winit::dpi::PhysicalSize;

use crate::world_renderer::camera::camera_state::{CameraState, CameraUniform};
use controls::{CameraController, CameraView};

pub mod camera_state;
pub mod controls;

/// The camera is responsible for managing the camera state and updating the camera buffer.
pub struct Camera<C: CameraController> {
    camera_state: CameraState,
    camera_uniform: CameraUniform,
    buffer: Buffer,
    bind_group: BindGroup,
    controller: C, // Maybe make self.controller a parameter in update_camera_buffer and not an attribute to loosen dependency?
}

impl<C: CameraController> Camera<C> {
    pub fn init(controller: C, device: &Device) -> (Self, BindGroupLayout) {
        let camera_state = CameraState::new(Vector2::new(1.0, 1.0));
        let camera_uniform = CameraUniform::new();

        let buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        (
            Self {
                camera_state,
                camera_uniform,
                buffer,
                bind_group,
                controller,
            },
            bind_group_layout,
        )
    }

    /// Update the view from controller inputs and write the camera buffer.
    pub fn update_camera_buffer(&mut self, queue: &Queue) {
        if let Some(view) = self.controller.get_view() {
            self.apply_view(&view)
        }

        self.camera_uniform.update_view_proj(&self.camera_state);

        queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    fn apply_view(&mut self, view: &CameraView) {
        self.camera_state.pos = view.position;
        self.camera_state.size = self.camera_state.size.normalize_to(1.0 / view.zoom);
    }

    /// Set the size of the view in pixels.
    pub fn set_view_size(&mut self, new_size: PhysicalSize<u32>) {
        let new_size = Vector2::new(new_size.width as f32, new_size.height as f32);
        self.camera_state.size = self.camera_state.size.project_on(new_size);
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}
