use cooltraption_assets::asset_bundle::AssetBundle;
use cooltraption_assets::texture_atlas::{TextureAtlas, TextureAtlasBuilder};
pub use cooltraption_assets::*;
use std::time::Duration;
use wgpu::util::DeviceExt;
use wgpu::*;
use winit::window::Window;

use crate::renderer::render_frame::RenderFrame;
use crate::renderer::wgpu_state::WgpuState;
use crate::renderer::{BoxedRenderer, RenderError, Renderer, RendererInitializer};
use crate::world_renderer::camera::controls::CameraController;
use crate::world_renderer::camera::Camera;
use crate::world_renderer::mesh::{Mesh, Vertex};
use crate::world_renderer::texture_atlas_resource::TextureAtlasResource;
use crate::world_renderer::world_state::{Drawable, WorldState};
pub use world_state::render_entity::{RenderEntity, RenderEntityRaw};

pub mod camera;
mod mesh;
mod texture_atlas_resource;
pub mod world_state;

struct WorldRenderer<C, I>
where
    C: CameraController,
    I: Iterator<Item = Vec<Drawable>>,
{
    render_pipeline: RenderPipeline,
    mesh: Mesh,
    instance_buffer: Buffer,
    texture_atlas_resource: TextureAtlasResource,
    assets: AssetBundle,
    camera: Camera<C>,
    state_recv: I,
    world_state: WorldState,
}

impl<C, I> Renderer for WorldRenderer<C, I>
where
    C: CameraController,
    I: Iterator<Item = Vec<Drawable>>,
{
    fn render(&mut self, render_frame: &mut RenderFrame) -> Result<(), Box<dyn RenderError>> {
        for drawables in self.state_recv.by_ref() {
            self.world_state.update(drawables);
        }

        let instances = self
            .world_state
            .get_render_entities(&self.texture_atlas_resource, &self.assets);

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
            .map(RenderEntity::to_raw)
            .collect::<Vec<_>>();
        let instance_data = bytemuck::cast_slice::<_, u8>(&instances_raw);

        if self.instance_buffer.size() < instance_data.len() as u64 {
            self.instance_buffer = create_instance_buffer(instance_data, render_frame.device);
        } else {
            render_frame
                .queue
                .write_buffer(&self.instance_buffer, 0, instance_data);
        }

        self.camera.update_camera_buffer(render_frame.queue);
        self.camera.set_view_size(render_frame.window.inner_size());

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, self.texture_atlas_resource.texture_bind_group(), &[]);
        render_pass.set_bind_group(1, self.camera.camera_bind_group(), &[]);
        render_pass.set_bind_group(2, self.texture_atlas_resource.region_bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.mesh.vertices().slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh.indices().slice(..), IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.mesh.num_indices(), 0, 0..instances.len() as _);

        Ok(())
    }
}

pub struct WorldRendererInitializer<C, I>
where
    C: CameraController,
    I: Iterator<Item = Vec<Drawable>>,
{
    pub texture_atlas_builder: TextureAtlasBuilder,
    pub fixed_delta_time: Duration,
    pub assets: AssetBundle,
    pub controller: C,
    pub state_recv: I,
}

impl<C, I> RendererInitializer for WorldRendererInitializer<C, I>
where
    C: CameraController + 'static,
    I: Iterator<Item = Vec<Drawable>> + 'static,
{
    fn init(self: Box<Self>, wgpu_state: &mut WgpuState, _window: &Window) -> BoxedRenderer {
        let (camera, camera_bind_group_layout) = Camera::init(self.controller, wgpu_state);

        let (texture_atlas_resource, atlas_texture_bind_group_layout, regions_bind_group_layout) =
            TextureAtlasResource::allocate(
                self.texture_atlas_builder,
                &wgpu_state.device,
                &wgpu_state.queue,
            );

        let mesh = Mesh::quad(&wgpu_state.device);

        let shader = wgpu_state
            .device
            .create_shader_module(include_wgsl!("shader.wgsl"));

        let instance_buffer = create_instance_buffer(&[0], &wgpu_state.device);

        let render_pipeline = create_pipeline(
            &wgpu_state.device,
            &wgpu_state.config.format,
            &[
                &atlas_texture_bind_group_layout,
                &camera_bind_group_layout,
                &regions_bind_group_layout,
            ],
            &shader,
        );

        Box::new(WorldRenderer {
            render_pipeline,
            mesh,
            instance_buffer,
            texture_atlas_resource,
            camera,
            assets: self.assets,
            state_recv: Box::new(self.state_recv),
            world_state: WorldState::new(self.fixed_delta_time),
        })
    }
}

fn create_instance_buffer(data: &[u8], device: &Device) -> Buffer {
    device.create_buffer_init(&util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: data,
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    })
}

fn create_pipeline(
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
            buffers: &[Vertex::desc(), RenderEntityRaw::desc()],
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
