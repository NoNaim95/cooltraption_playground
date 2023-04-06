use std::cell::RefCell;
use std::rc::Rc;

use wgpu::util::DeviceExt;
use wgpu::*;
use winit::event::Event;

use crate::asset_bundle::{AssetBundle, TextureAtlas, TextureAtlasBuilder};
use crate::camera::Camera;
pub use crate::render::instance_renderer::render_instance::{RenderInstance, RenderInstanceRaw};
pub use crate::render::instance_renderer::world_state::WorldState;
use crate::render::render_frame::RenderFrame;
use crate::render::vertex::{Vertex, INDICES, VERTICES};
use crate::render::{Renderer, RendererInitializer};
use crate::window::event_handler::{Context, EventHandler};
use crate::window::CooltraptionEvent;

mod render_instance;
pub mod world_state;

pub trait States: FnMut() -> Option<WorldState> {}
impl<F: FnMut() -> Option<WorldState>> States for F {}

struct InstanceRenderer<S: States> {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    texture_bind_group: BindGroup,
    instance_buffer: Buffer,
    texture_atlas: TextureAtlas,
    camera: Camera,
    assets: AssetBundle,
    states: S,
    world_state: [WorldState; 2],
}

pub struct InstanceRendererInitializer<S: States> {
    pub texture_atlas_builder: TextureAtlasBuilder,
    pub assets: AssetBundle,
    pub states: S,
}

impl<S: States> EventHandler for InstanceRenderer<S> {
    fn handle_event(&mut self, event: &Event<CooltraptionEvent>, context: &mut Context) {
        self.camera.handle_event(event, context);
    }
}

impl<S: States> Renderer for InstanceRenderer<S> {
    fn render(&mut self, render_frame: &mut RenderFrame) {
        while let Some(state) = (self.states)() {
            self.update_state(state);
        }

        let instances = self.world_state[1].interpolate(
            &self.world_state[0],
            &self.assets,
            &self.texture_atlas,
        );

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

        let instances_raw = instances
            .iter()
            .map(RenderInstance::to_raw)
            .collect::<Vec<_>>();
        let instance_data = bytemuck::cast_slice::<_, u8>(&instances_raw);

        if self.instance_buffer.size() < instance_data.len() as u64 {
            self.instance_buffer = create_instance_buffer(instance_data, render_frame.device);
        } else {
            render_frame
                .queue
                .write_buffer(&self.instance_buffer, 0, instance_data);
        }

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
        render_pass.set_bind_group(1, self.camera.camera_bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.num_indices, 0, 0..instances.len() as _);
    }
}

impl<S: States> InstanceRenderer<S> {
    fn update_state(&mut self, new_state: WorldState) {
        self.world_state.swap(0, 1);
        self.world_state[0] = new_state;
    }
}

fn create_instance_buffer(data: &[u8], device: &Device) -> Buffer {
    device.create_buffer_init(&util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: data,
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    })
}

impl<S: States + 'static> RendererInitializer for InstanceRendererInitializer<S> {
    fn init(self: Box<Self>, context: &mut Context) -> Rc<RefCell<dyn Renderer>> {
        let wgpu_state = &context.wgpu_state;

        let texture_atlas = self
            .texture_atlas_builder
            .build(&wgpu_state.device, &wgpu_state.queue);

        let camera = Camera::init(wgpu_state);

        let texture_bind_group_layout =
            wgpu_state
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Texture {
                                multisampled: false,
                                view_dimension: TextureViewDimension::D2,
                                sample_type: TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT,
                            // This should match the filterable field of the
                            // corresponding Texture entry above.
                            ty: BindingType::Sampler(SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        let texture_bind_group = wgpu_state.device.create_bind_group(&BindGroupDescriptor {
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
            label: Some("texture_bind_group"),
        });

        let shader = wgpu_state
            .device
            .create_shader_module(include_wgsl!("shader.wgsl"));

        let instance_buffer = create_instance_buffer(&[0], &wgpu_state.device);

        let render_pipeline = Self::create_pipeline(
            &wgpu_state.device,
            &wgpu_state.config.format,
            &[
                &texture_bind_group_layout,
                camera.camera_bind_group_layout(),
            ],
            &shader,
        );

        let vertex_buffer = wgpu_state
            .device
            .create_buffer_init(&util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: BufferUsages::VERTEX,
            });

        let index_buffer = wgpu_state
            .device
            .create_buffer_init(&util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: BufferUsages::INDEX,
            });
        let num_indices = INDICES.len() as u32;

        let instance_renderer = Rc::new(RefCell::new(InstanceRenderer {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            texture_bind_group,
            instance_buffer,
            texture_atlas,
            camera,
            assets: self.assets,
            states: self.states,
            world_state: [Default::default(), Default::default()],
        }));

        context.event_handlers.push(instance_renderer.clone());

        instance_renderer
    }
}

impl<S: States> InstanceRendererInitializer<S> {
    pub fn create_pipeline(
        device: &Device,
        format: &TextureFormat,
        bind_groups: &[&BindGroupLayout],
        shader: &ShaderModule,
    ) -> RenderPipeline {
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: bind_groups,
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                // TODO: Load shaders from assets
                module: shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), RenderInstanceRaw::desc()],
            },
            fragment: Some(FragmentState {
                module: shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: *format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }
}
