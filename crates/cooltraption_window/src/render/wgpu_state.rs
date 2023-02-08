use cgmath::prelude::*;
use cgmath::{Deg, Quaternion, Vector3};
use fixed::FixedI64;
use guillotiere::euclid::default::{Rect, Size2D};
use guillotiere::AtlasAllocator;
use log::warn;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use wgpu::util::DeviceExt;
use wgpu::{
    util, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType,
    BufferUsages, Color, CommandEncoderDescriptor, Device, IndexFormat, LoadOp, Operations, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, ShaderStages, Surface,
    SurfaceConfiguration, SurfaceError, TextureViewDescriptor,
};

use crate::render::camera::{Camera, CameraUniform};
use crate::render::instance::Instance;
use crate::render::render_batch::RenderBatch;
use crate::render::uninitialized_wgpu_state::UninitializedWgpuState;
use crate::render::{Drawable, Position};

pub struct WgpuState {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
    camera_bind_group_layout: BindGroupLayout,
    render_batch: RenderBatch,
}

impl From<UninitializedWgpuState> for WgpuState {
    fn from(uninitialized_state: UninitializedWgpuState) -> Self {
        let UninitializedWgpuState {
            surface,
            queue,
            config,
            size,
            device,
        } = uninitialized_state;

        let camera = Camera {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 1.0, 2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            z_near: 0.1,
            z_far: 100.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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

        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_bind_group_layout,
            render_batch,
        }
    }
}

impl WgpuState {
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render_all(&mut self, drawables: &[(Position, Drawable)]) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        if let Some(render_set) = self.render_sets.first_mut() {
            let mut encoder = self
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            {
                let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });

                render_set.instances = drawables
                    .iter()
                    .map(|(p, _)| Instance {
                        position: Vector3::new(FixedI64::to_num(p.x), FixedI64::to_num(p.y), 0.0),
                        rotation: Quaternion::zero(),
                    })
                    .collect();

                let instance_data = render_set
                    .instances
                    .iter()
                    .map(Instance::to_raw)
                    .collect::<Vec<_>>();
                render_set.instance_buffer =
                    self.device.create_buffer_init(&util::BufferInitDescriptor {
                        label: Some("Instance Buffer"),
                        contents: bytemuck::cast_slice(&instance_data),
                        usage: BufferUsages::VERTEX,
                    });

                render_pass.set_pipeline(&render_set.render_pipeline);
                render_pass.set_bind_group(0, &render_set.diffuse_bind_group, &[]);
                render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
                render_pass.set_vertex_buffer(0, render_set.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, render_set.instance_buffer.slice(..));
                render_pass
                    .set_index_buffer(render_set.index_buffer.slice(..), IndexFormat::Uint16);

                render_pass.draw_indexed(
                    0..render_set.num_indices,
                    0,
                    0..render_set.instances.len() as _,
                );
            }

            self.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        } else {
            warn!("No render sets to render!");
        }

        Ok(())
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn device_mut(&mut self) -> &mut Device {
        &mut self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn queue_mut(&mut self) -> &mut Queue {
        &mut self.queue
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }
}
