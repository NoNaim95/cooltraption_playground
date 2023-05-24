mod ellipse;
mod rect;
mod vertex;

use crate::world_renderer::camera::controls::CameraController;
use crate::world_renderer::camera::Camera;
use crate::world_renderer::mesh::Mesh;
use egui::epaint::ahash::{HashMap, HashMapExt};
pub use ellipse::{ellipse, Ellipse};
pub use rect::{rect, Rect};
use std::f32::MAX;
use uuid::Uuid;
use wgpu::util::DeviceExt;
use wgpu::{
    util, BindGroupLayout, Buffer, BufferUsages, Device, IndexFormat, Queue, RenderPass,
    RenderPipeline, TextureFormat,
};

#[macro_export]
macro_rules! unique_id {
    () => {{
        use lazy_static::lazy_static;
        use uuid::Uuid;
        lazy_static! {
            static ref UNIQUE_ID: Uuid = Uuid::new_v4();
        }
        *UNIQUE_ID
    }};
}

pub type Coord = (f32, f32);
pub type Size = (f32, f32);
pub type Age = f32;

#[derive(Copy, Clone)]
pub enum BoundingBox {
    Corners(Coord, Coord),
    Origin(Origin, Size),
}

impl BoundingBox {
    fn top_left(&self) -> Coord {
        match self {
            BoundingBox::Corners(p1, p2) => (p1.0.min(p2.0), p1.1.min(p2.1)),
            BoundingBox::Origin(origin, size) => {
                let (x, y) = match origin {
                    Origin::TopLeft(coord) => *coord,
                    Origin::TopRight(coord) => (coord.0 - size.0, coord.1),
                    Origin::BottomLeft(coord) => (coord.0, coord.1 - size.1),
                    Origin::BottomRight(coord) => (coord.0 - size.0, coord.1 - size.1),
                    Origin::Center(coord) => (coord.0 - size.0 / 2.0, coord.1 - size.1 / 2.0),
                };
                (x, y)
            }
        }
    }

    fn size(&self) -> Size {
        match self {
            BoundingBox::Corners(p1, p2) => ((p1.0 - p2.0).abs(), (p1.1 - p2.1).abs()),
            BoundingBox::Origin(_, size) => *size,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Origin {
    TopLeft(Coord),
    TopRight(Coord),
    BottomLeft(Coord),
    BottomRight(Coord),
    Center(Coord),
}

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
    Rect(Uuid, Age, Rect),
    Ellipse(Uuid, Age, Ellipse),
}

impl Shape {
    pub fn to_raw(&self) -> ShapeRaw {
        match self {
            Shape::Rect(_, age, rect) => rect.to_raw(*age),
            Shape::Ellipse(_, age, ellipse) => ellipse.to_raw(*age),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShapeRaw {
    transform: [[f32; 4]; 4],
    color: [f32; 4],
}

impl ShapeRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
        1 => Float32x4,
        2 => Float32x4,
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
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
    gizmos: HashMap<Uuid, Shape>,
}

impl GizmoStage {
    pub fn render<'a, 'b: 'a, C: CameraController>(
        &'b mut self,
        render_pass: &mut RenderPass<'a>,
        device: &Device,
        queue: &Queue,
        camera: &'a Camera<C>,
    ) {
        // Update gizmo ages
        self.gizmos.values_mut().for_each(|gizmo| match gizmo {
            Shape::Rect(_, age, _) => {
                *age += 0.001;
            }
            Shape::Ellipse(_, age, _) => {
                *age += 0.001;
            }
        });

        let shapes_raw = self.gizmos.values().map(Shape::to_raw).collect::<Vec<_>>();
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

    pub fn clear_old(&mut self) {
        const MAX_AGE: f32 = 1.0;
        self.gizmos.retain(|_, shape| match shape {
            Shape::Rect(_, age, _) => *age < MAX_AGE,
            Shape::Ellipse(_, age, _) => *age < MAX_AGE,
        });
    }
}

impl GizmoStage {
    pub fn new(mesh: Mesh, pipeline: RenderPipeline, instance_buffer: Buffer) -> Self {
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
    pub fn new(device: &Device, format: &TextureFormat, camera_bgl: &BindGroupLayout) -> Self {
        Self {
            stages: vec![
                GizmoStage::new(
                    Mesh::new(device, rect::VERTICES, rect::INDICES, "Rect Gizmo"),
                    rect::create_pipeline(device, format, camera_bgl),
                    create_instance_buffer(&[], device, "Rect"),
                ),
                GizmoStage::new(
                    Mesh::new(device, ellipse::VERTICES, ellipse::INDICES, "Ellipse Gizmo"),
                    ellipse::create_pipeline(device, format, camera_bgl),
                    create_instance_buffer(&[], device, "Ellipse"),
                ),
            ],
        }
    }

    pub fn add_rect(&mut self, uuid: Uuid, rect: Rect) {
        self.stages[0]
            .gizmos
            .insert(uuid, Shape::Rect(uuid, 0.0, rect));
    }

    pub fn add_ellipse(&mut self, uuid: Uuid, ellipse: Ellipse) {
        self.stages[1]
            .gizmos
            .insert(uuid, Shape::Ellipse(uuid, 0.0, ellipse));
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
