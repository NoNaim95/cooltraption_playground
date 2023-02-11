use crate::render::instance::{Instance, InstanceRaw};
use crate::render::texture_atlas::TextureAtlas;
use crate::render::vertex::{INDICES, VERTICES};
use crate::render::wgpu_state::WgpuState;
use wgpu::util::DeviceExt;
use wgpu::{
    include_wgsl, util, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferUsages, Color,
    CommandEncoderDescriptor, IndexFormat, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, ShaderStages, SurfaceError, TextureViewDescriptor,
};

pub struct InstanceRenderer {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    diffuse_bind_group: BindGroup,
    instance_buffer: Buffer,
    texture_atlas: TextureAtlas,
}

impl InstanceRenderer {
    pub fn new(state: &WgpuState, texture_atlas: TextureAtlas) -> Self {
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

        let instance_data: Vec<InstanceRaw> = vec![];
        let instance_buffer = state
            .device
            .create_buffer_init(&util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: BufferUsages::VERTEX,
            });

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
        }
    }

    pub fn render_all(
        &mut self,
        instances: &[Instance],
        wgpu_state: &WgpuState,
    ) -> Result<(), SurfaceError> {
        let output = wgpu_state.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = wgpu_state
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

            let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
            self.instance_buffer =
                wgpu_state
                    .device
                    .create_buffer_init(&util::BufferInitDescriptor {
                        label: Some("Instance Buffer"),
                        contents: bytemuck::cast_slice(&instance_data),
                        usage: BufferUsages::VERTEX,
                    });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &wgpu_state.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..instances.len() as _);
        }

        wgpu_state.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn texture_atlas(&self) -> &TextureAtlas {
        &self.texture_atlas
    }
}
