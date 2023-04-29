use cgmath::Vector3;
use wgpu::util::DeviceExt;
use wgpu::*;
use winit::dpi::PhysicalSize;

use crate::renderer::wgpu_state::WgpuState;
use crate::world_renderer::camera::camera_state::{CameraState, CameraUniform};
use crate::world_renderer::camera::controls::{CameraController, CameraControls};

pub mod camera_state;
pub mod controls;

pub struct Camera<C: CameraController> {
    camera_state: CameraState,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
    camera_bind_group_layout: BindGroupLayout,
    controller: C,
}

impl<C: CameraController> Camera<C> {
    pub fn init(controller: C, wgpu_state: &WgpuState) -> Self {
        let camera_state =
            CameraState::new((wgpu_state.size.width as f32, wgpu_state.size.height as f32).into());
        let camera_uniform = CameraUniform::new();

        let camera_buffer = wgpu_state
            .device
            .create_buffer_init(&util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });

        let camera_bind_group_layout =
            wgpu_state
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
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

        let camera_bind_group = wgpu_state.device.create_bind_group(&BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            camera_state,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_bind_group_layout,
            controller,
        }
    }

    pub fn update_camera_buffer(&mut self, queue: &Queue) {
        if let Some(controls) = self.controller.get_controls() {
            self.apply_controls(&controls)
        }

        self.camera_uniform.update_view_proj(&self.camera_state);

        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    fn apply_controls(&mut self, controls: &CameraControls) {
        self.camera_state.target += Vector3::new(controls.move_vec.x, controls.move_vec.y, 0.0);
        self.camera_state.zoom *= controls.zoom;
    }

    pub fn set_view_size(&mut self, size: PhysicalSize<u32>) {
        self.camera_state.aspect = size.width as f32 / size.height as f32;
    }

    pub fn camera_bind_group(&self) -> &BindGroup {
        &self.camera_bind_group
    }

    pub fn camera_bind_group_layout(&self) -> &BindGroupLayout {
        &self.camera_bind_group_layout
    }
}