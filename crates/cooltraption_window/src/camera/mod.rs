use cgmath::Vector3;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages,
    Queue, ShaderStages, util,
};
use wgpu::util::DeviceExt;
use winit::event::{Event, WindowEvent};

use controller::CameraControls;

use crate::{Context, CooltraptionEvent, EventHandler, WgpuState};
use crate::camera::camera_state::{CameraState, CameraUniform};

pub mod camera_state;
pub mod controller;

pub struct Camera {
    camera_state: CameraState,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
    camera_bind_group_layout: BindGroupLayout,
}

impl EventHandler for Camera {
    fn handle_event(&mut self, event: &Event<CooltraptionEvent>, context: &mut Context) {
        match event {
            Event::WindowEvent { event, window_id } => {
                if window_id != &context.window.id() {
                    return;
                }

                match event {
                    WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                        self.camera_state.aspect = context.wgpu_state.aspect();
                    }
                    _ => {}
                }
            }
            Event::UserEvent(event) => match event {
                CooltraptionEvent::Render => self.update_camera_buffer(&context.wgpu_state.queue),
                CooltraptionEvent::CameraControls(controls) => self.apply_controls(controls),
                _ => {}
            },
            _ => {}
        }
    }
}

impl Camera {
    pub fn new(wgpu_state: &WgpuState) -> Self {
        let camera_state = CameraState::new(wgpu_state.aspect());
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
        }
    }

    pub fn apply_controls(&mut self, controls: &CameraControls) {
        self.camera_state.target += Vector3::new(controls.move_vec.x, controls.move_vec.y, 0.0);
        self.camera_state.zoom *= controls.zoom;
    }

    pub fn update_camera_buffer(&mut self, queue: &Queue) {
        self.camera_uniform.update_view_proj(&self.camera_state);

        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn camera_bind_group(&self) -> &BindGroup {
        &self.camera_bind_group
    }

    pub fn camera_bind_group_layout(&self) -> &BindGroupLayout {
        &self.camera_bind_group_layout
    }
}
