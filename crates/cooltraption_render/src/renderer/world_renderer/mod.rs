use cgmath::Point2;
use cooltraption_assets::asset_bundle::Asset::Strings;
use cooltraption_assets::asset_bundle::AssetBundle;
use cooltraption_assets::texture_atlas::TextureAtlasBuilder;
pub use cooltraption_assets::*;
use std::time::Duration;
use wgpu::util::DeviceExt;
use wgpu::*;
use winit::window::Window;

use crate::renderer::render_frame::RenderFrame;
use crate::renderer::wgpu_state::WgpuState;
use crate::renderer::{BoxedRenderer, RenderError, Renderer, RendererInitializer};
use crate::unique_id;
use crate::world_renderer::camera::controls::CameraController;
use crate::world_renderer::camera::Camera;
use crate::world_renderer::gizmos::Origin;
use crate::world_renderer::gpu_texture_atlas::GpuTextureAtlas;
use crate::world_renderer::mesh::{Mesh, Vertex};
use crate::world_renderer::world_state::{Drawable, Transform, WorldState};
pub use world_state::render_entity::{RenderEntity, RenderEntityRaw};

pub mod camera;
pub mod gizmos;
mod gpu_texture_atlas;
mod mesh;
pub mod world_state;

struct WorldRenderer<C, I>
where
    C: CameraController,
    I: Iterator<Item = Vec<Drawable>>,
{
    render_pipeline: RenderPipeline,
    mesh: Mesh,
    instance_buffer: Buffer,
    gpu_texture_atlas: GpuTextureAtlas,
    assets: AssetBundle,
    camera: Camera<C>,
    world_state: WorldState<I>,
}

impl<C, I> Renderer for WorldRenderer<C, I>
where
    C: CameraController,
    I: Iterator<Item = Vec<Drawable>>,
{
    fn render(&mut self, render_frame: &mut RenderFrame) -> Result<(), Box<dyn RenderError>> {
        let entities = self
            .world_state
            .create_entities(&self.gpu_texture_atlas, &self.assets);

        let clear_color = try_get_background(&self.assets).unwrap_or(Color::RED);

        {
            let mut render_pass = render_frame
                .encoder
                .begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &render_frame.view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(clear_color),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });

            let entities_raw = entities
                .iter()
                .map(RenderEntity::to_raw)
                .collect::<Vec<_>>();
            let entities_data = bytemuck::cast_slice::<_, u8>(&entities_raw);

            if self.instance_buffer.size() < entities_data.len() as u64 {
                self.instance_buffer = create_instance_buffer(entities_data, render_frame.device);
            } else {
                render_frame
                    .queue
                    .write_buffer(&self.instance_buffer, 0, entities_data);
            }

            self.camera.update_camera_buffer(render_frame.queue);
            self.camera.set_view_size(render_frame.window.inner_size());

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, self.gpu_texture_atlas.texture_bind_group(), &[]);
            render_pass.set_bind_group(1, self.camera.bind_group(), &[]);
            render_pass.set_bind_group(2, self.gpu_texture_atlas.region_bind_group(), &[]);
            render_pass.set_vertex_buffer(0, self.mesh.vertices().slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.mesh.indices().slice(..), IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.mesh.num_indices(), 0, 0..entities.len() as _);
        }

        gizmos::rect(
            unique_id!(),
            gizmos::BoundingBox::Corners((-1.0, -0.5), (1.0, 0.5)),
        );
        gizmos::rect(
            unique_id!(),
            gizmos::BoundingBox::Corners((-0.5, -1.0), (0.5, 1.0)),
        );
        gizmos::rect(
            unique_id!(),
            gizmos::BoundingBox::Origin(Origin::Center((0.0, 0.0)), (0.5, 0.5)),
        );

        gizmos::render_all(
            &mut render_frame.encoder,
            &render_frame.view,
            render_frame.device,
            render_frame.queue,
            &self.camera,
        );

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
        let (camera, camera_bgl) = Camera::init(self.controller, &wgpu_state.device);

        let (gpu_texture_atlas, bind_group_layouts) = GpuTextureAtlas::allocate(
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
                &bind_group_layouts.texture,
                &camera_bgl,
                &bind_group_layouts.region,
            ],
            &shader,
        );

        gizmos::init(&wgpu_state.device, &wgpu_state.config.format, &camera_bgl);

        Box::new(WorldRenderer {
            render_pipeline,
            mesh,
            instance_buffer,
            gpu_texture_atlas,
            camera,
            assets: self.assets,
            world_state: WorldState::new(self.state_recv, self.fixed_delta_time),
        })
    }
}

fn try_get_background(assets: &AssetBundle) -> Option<Color> {
    if let Some(Strings(background)) = assets.get_asset("background") {
        let r = background.map.get("red")?.parse::<f64>().ok()?;
        let g = background.map.get("green")?.parse::<f64>().ok()?;
        let b = background.map.get("blue")?.parse::<f64>().ok()?;
        let a = background.map.get("alpha")?.parse::<f64>().ok()?;

        Some(Color { r, g, b, a })
    } else {
        None
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
