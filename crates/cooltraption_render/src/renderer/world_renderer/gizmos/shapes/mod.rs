mod rect;
mod vertex;

use crate::world_renderer::camera::controls::CameraController;
use crate::world_renderer::camera::Camera;
use crate::world_renderer::mesh::Mesh;
pub use rect::{rect, Rect};
use wgpu::util::DeviceExt;
use wgpu::{
    util, BindGroupLayout, Buffer, BufferUsages, Device, IndexFormat, Queue, RenderPass,
    RenderPipeline, TextureFormat,
};

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::new(1.0, 0.0, 1.0);
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0);
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0);

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

pub enum Shape {
    Rect(Rect),
}

impl Shape {
    pub fn to_raw(&self) -> ShapeRaw {
        match self {
            Shape::Rect(rect) => rect.to_raw(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShapeRaw {
    transform: [[f32; 4]; 4],
    color: [f32; 3],
}

impl ShapeRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
        1 => Float32x4,
        2 => Float32x4,
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x3,
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct GizmoStage {
    pipeline: RenderPipeline,
    mesh: Mesh,
    instance_buffer: Buffer,
    gizmos: Vec<Shape>,
}

impl GizmoStage {
    pub fn render<'a, 'b: 'a, C: CameraController>(
        &'b mut self,
        render_pass: &mut RenderPass<'a>,
        device: &Device,
        queue: &Queue,
        camera: &'a Camera<C>,
    ) {
        let shapes_raw = self.gizmos.iter().map(Shape::to_raw).collect::<Vec<_>>();
        let shapes_data = bytemuck::cast_slice::<_, u8>(&shapes_raw);

        if self.instance_buffer.size() < shapes_data.len() as u64 {
            self.instance_buffer = create_instance_buffer(shapes_data, device, "Rect");
        } else {
            queue.write_buffer(&self.instance_buffer, 0, shapes_data);
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.mesh.vertices().slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh.indices().slice(..), IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.mesh.num_indices(), 0, 0..self.gizmos.len() as _);
    }

    pub fn clear(&mut self) {
        self.gizmos.clear();
    }
}

impl GizmoStage {
    pub fn new(mesh: Mesh, pipeline: RenderPipeline, instance_buffer: Buffer) -> Self {
        Self {
            pipeline,
            mesh,
            instance_buffer,
            gizmos: vec![],
        }
    }
}

pub struct GizmoStages {
    stages: Vec<GizmoStage>,
}

impl GizmoStages {
    pub fn new(device: &Device, format: &TextureFormat, camera_bgl: &BindGroupLayout) -> Self {
        Self {
            stages: vec![GizmoStage::new(
                Mesh::new(device, rect::VERTICES, rect::INDICES, "Rect Gizmo"),
                rect::create_pipeline(device, format, camera_bgl),
                create_instance_buffer(&[], device, "Rect"),
            )],
        }
    }

    pub fn add_rect(&mut self, rect: Rect) {
        self.stages[0].gizmos.push(Shape::Rect(rect));
    }

    pub fn render<'a, 'b: 'a, C: CameraController>(
        &'b mut self,
        render_pass: &mut RenderPass<'a>,
        device: &Device,
        queue: &Queue,
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

pub fn create_instance_buffer(data: &[u8], device: &Device, label: &'static str) -> Buffer {
    device.create_buffer_init(&util::BufferInitDescriptor {
        label: Some(format!("{} Gizmo Instance Buffer", label).as_str()),
        contents: data,
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    })
}
