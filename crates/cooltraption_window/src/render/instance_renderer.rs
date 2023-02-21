use crate::asset_bundle::AssetBundle;
use std::sync::mpsc::Receiver;
use wgpu::util::DeviceExt;
use wgpu::{
    include_wgsl, util, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferUsages, Color,
    CommandEncoderDescriptor, Device, IndexFormat, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline, ShaderStages,
    SurfaceError, TextureViewDescriptor,
};

use crate::render::instance::Instance;
use crate::render::render_frame::RenderFrame;
use crate::render::texture_atlas::TextureAtlas;
use crate::render::vertex::{INDICES, VERTICES};
use crate::render::wgpu_state::WgpuState;
use crate::render::{Renderer, WorldState};

pub struct InstanceRenderer {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    diffuse_bind_group: BindGroup,
    instance_buffer: Buffer,
    assets: AssetBundle,
    texture_atlas: TextureAtlas,
    state_recv: Receiver<WorldState>,
    world_state: [WorldState; 2],
}

impl Renderer for InstanceRenderer {
    fn render(&mut self, render_frame: &mut RenderFrame) {
        while let Ok(state) = self.state_recv.try_recv() {
            self.update_state(state);
        }

        let instances = self.world_state[1].interpolate(
            &self.world_state[0],
            &self.assets,
            &self.texture_atlas,
        );

        self.do_render_pass(instances.as_slice(), render_frame, &render_frame.camera);
    }
}

impl InstanceRenderer {
    pub fn new(
        state: &WgpuState,
        assets: AssetBundle,
        texture_atlas: TextureAtlas,
        state_recv: Receiver<WorldState>,
    ) -> Self {
        let texture_bind_group_layout =
            state
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT,
                            // This should match the filterable field of the
                            // corresponding Texture entry above.
                            ty: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        let diffuse_bind_group = state.device.create_bind_group(&BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(texture_atlas.view()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(texture_atlas.sampler()),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let shader = state
            .device
            .create_shader_module(include_wgsl!("shader.wgsl"));

        let instance_buffer = Self::create_instance_buffer(&[0], &state.device);

        let render_pipeline = state.create_pipeline(
            &[&texture_bind_group_layout, &state.camera_bind_group_layout],
            &shader,
        );

        let vertex_buffer = state
            .device
            .create_buffer_init(&util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: BufferUsages::VERTEX,
            });

        let index_buffer = state
            .device
            .create_buffer_init(&util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: BufferUsages::INDEX,
            });
        let num_indices = INDICES.len() as u32;

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            instance_buffer,
            texture_atlas,
            assets,
            state_recv,
            world_state: [WorldState::default(), WorldState::default()],
        }
    }

    fn update_state(&mut self, new_state: WorldState) {
        self.world_state.swap(0, 1);
        self.world_state[0] = new_state;
    }

    fn do_render_pass(
        &mut self,
        instances: &[Instance],
        render_frame: &mut RenderFrame,
        camera_bind_group: &BindGroup,
    ) {
        let mut render_pass = render_frame
            .encoder
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &render_frame.view,
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

        let instances_raw = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_data = bytemuck::cast_slice::<_, u8>(&instances_raw);

        if self.instance_buffer.size() < instance_data.len() as u64 {
            self.instance_buffer = Self::create_instance_buffer(instance_data, render_frame.device);
        } else {
            render_frame
                .queue
                .write_buffer(&self.instance_buffer, 0, instance_data);
        }

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_bind_group(1, camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.num_indices, 0, 0..instances.len() as _);
    }

    fn create_instance_buffer(data: &[u8], device: &Device) -> Buffer {
        device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: data,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        })
    }
}
