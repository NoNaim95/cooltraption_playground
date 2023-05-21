mod shapes;

use lazy_static::lazy_static;
use std::sync::Mutex;
use wgpu::{
    include_wgsl, BindGroupLayout, CommandEncoder, Device, LoadOp, Operations,
    PipelineLayoutDescriptor, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, TextureFormat, TextureView, VertexState,
};

use crate::world_renderer::camera::controls::CameraController;
use crate::world_renderer::camera::Camera;
pub use shapes::*;

lazy_static! {
    static ref GIZMOS: Mutex<Option<GizmoStages>> = Mutex::new(None);
}

pub fn init(device: &Device, format: &TextureFormat, camera_bgl: &BindGroupLayout) {
    let gizmos = GizmoStages::new(device, format, camera_bgl);
    GIZMOS.lock().expect("gizmo mutex").replace(gizmos);
}

pub fn render_all<'a, 'b: 'a, C: CameraController>(
    encoder: &mut CommandEncoder,
    view: &TextureView,
    device: &Device,
    queue: &Queue,
    camera: &'a Camera<C>,
) {
    let mut gizmos = GIZMOS.lock().expect("gizmo mutex");
    let gizmos = gizmos.as_mut().expect("initialized gizmos");
    {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Gizmo Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
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
        stage.clear();
    }
}
