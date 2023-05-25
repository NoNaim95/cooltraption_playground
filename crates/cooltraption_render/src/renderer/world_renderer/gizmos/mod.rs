mod gizmo;
mod macros;
mod util;
mod vertex;

use crate::world_renderer::camera::controls::CameraController;
use crate::world_renderer::camera::Camera;
use crate::world_renderer::gizmos::gizmo::MAX_AGE;
pub use crate::world_renderer::gizmos::gizmo::{BoundingBox, Color, Gizmo, Origin, Shape};
use crate::world_renderer::gizmos::util::{
    create_instance_buffer, create_pipeline, INDICES, VERTICES,
};
use crate::world_renderer::mesh::Mesh;
use egui::epaint::ahash::{HashMap, HashMapExt};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::Instant;
use uuid::Uuid;

lazy_static! {
    static ref GIZMOS: Mutex<Option<GizmoStages>> = Mutex::new(None);
}

pub fn shape(uuid: Uuid, shape: Shape, color: Color, bounding_box: BoundingBox) {
    if let Some(gizmos) = GIZMOS.lock().expect("gizmo mutex").as_mut() {
        gizmos.add_gizmo(Gizmo::new(uuid, Instant::now(), bounding_box, color, shape));
    }
}

pub fn init(
    device: &wgpu::Device,
    format: &wgpu::TextureFormat,
    camera_bgl: &wgpu::BindGroupLayout,
) {
    let gizmos = GizmoStages::new(device, format, camera_bgl);
    GIZMOS.lock().expect("gizmo mutex").replace(gizmos);
}

pub fn render_all<'a, 'b: 'a, C: CameraController>(
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    camera: &'a Camera<C>,
) {
    let mut gizmos = GIZMOS.lock().expect("gizmo mutex");
    let gizmos = gizmos.as_mut().expect("initialized gizmos");
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Gizmo Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        for stage in gizmos.stages_mut() {
            stage.render(&mut render_pass, device, queue, camera);
        }
    }

    for stage in gizmos.stages_mut() {
        stage.clear_old();
    }
}

pub struct GizmoStage {
    pipeline: wgpu::RenderPipeline,
    mesh: Mesh,
    instance_buffer: wgpu::Buffer,
    gizmos: HashMap<Uuid, Gizmo>,
}

impl GizmoStage {
    pub fn render<'a, 'b: 'a, C: CameraController>(
        &'b mut self,
        render_pass: &mut wgpu::RenderPass<'a>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        camera: &'a Camera<C>,
    ) {
        let gizmos_raw = self.gizmos.values().map(Gizmo::to_raw).collect::<Vec<_>>();
        let gizmos_data = bytemuck::cast_slice::<_, u8>(&gizmos_raw);

        if self.instance_buffer.size() < gizmos_data.len() as u64 {
            self.instance_buffer = create_instance_buffer(gizmos_data, device, "Rect");
        } else {
            queue.write_buffer(&self.instance_buffer, 0, gizmos_data);
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.mesh.vertices().slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh.indices().slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.mesh.num_indices(), 0, 0..self.gizmos.len() as _);
    }

    pub fn clear_old(&mut self) {
        self.gizmos.retain(|_, gizmo| gizmo.age() < MAX_AGE);
    }
}

impl GizmoStage {
    pub fn new(mesh: Mesh, pipeline: wgpu::RenderPipeline, instance_buffer: wgpu::Buffer) -> Self {
        Self {
            pipeline,
            mesh,
            instance_buffer,
            gizmos: HashMap::new(),
        }
    }
}

pub struct GizmoStages {
    stages: Vec<GizmoStage>,
}

impl GizmoStages {
    pub fn new(
        device: &wgpu::Device,
        format: &wgpu::TextureFormat,
        camera_bgl: &wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            stages: vec![
                GizmoStage::new(
                    Mesh::new(device, VERTICES, INDICES, "Rect Gizmo"),
                    create_pipeline(
                        device,
                        format,
                        camera_bgl,
                        wgpu::include_wgsl!("shaders/rect.wgsl"),
                        "Rect",
                    ),
                    create_instance_buffer(&[], device, "Rect"),
                ),
                GizmoStage::new(
                    Mesh::new(device, VERTICES, INDICES, "Ellipse Gizmo"),
                    create_pipeline(
                        device,
                        format,
                        camera_bgl,
                        wgpu::include_wgsl!("shaders/ellipse.wgsl"),
                        "Ellipse",
                    ),
                    create_instance_buffer(&[], device, "Ellipse"),
                ),
            ],
        }
    }

    pub fn add_gizmo(&mut self, gizmo: Gizmo) {
        match gizmo.shape() {
            Shape::Rect => self.stages[0].gizmos.insert(gizmo.uuid(), gizmo),
            Shape::Ellipse => self.stages[1].gizmos.insert(gizmo.uuid(), gizmo),
        };
    }

    pub fn render<'a, 'b: 'a, C: CameraController>(
        &'b mut self,
        render_pass: &mut wgpu::RenderPass<'a>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        camera: &'a Camera<C>,
    ) {
        for stage in &mut self.stages {
            stage.render(render_pass, device, queue, camera);
        }
    }

    pub fn stages_mut(&mut self) -> &mut Vec<GizmoStage> {
        &mut self.stages
    }
}
