mod ellipse;
mod rect;
mod vertex;

use crate::world_renderer::camera::controls::CameraController;
use crate::world_renderer::camera::Camera;
use crate::world_renderer::mesh::Mesh;
use cgmath::{Matrix4, Vector3};
use egui::epaint::ahash::{HashMap, HashMapExt};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use uuid::Uuid;
use wgpu::util::DeviceExt;
use wgpu::{
    util, BindGroupLayout, Buffer, BufferUsages, CommandEncoder, Device, IndexFormat, LoadOp,
    Operations, Queue, RenderPass, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    TextureFormat, TextureView,
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

#[macro_export]
macro_rules! rect {
    ($bounding_box:expr, $color:expr) => {
        $crate::world_renderer::gizmos::shape(unique_id!(), Shape::Rect, $color, $bounding_box)
    };
}

#[macro_export]
macro_rules! ellipse {
    ($bounding_box:expr, $color:expr) => {
        $crate::world_renderer::gizmos::shape(unique_id!(), Shape::Ellipse, $color, $bounding_box)
    };
}

pub fn shape(uuid: Uuid, shape: Shape, color: Color, bounding_box: BoundingBox) {
    if let Some(gizmos) = GIZMOS.lock().expect("gizmo mutex").as_mut() {
        gizmos.add_gizmo(Gizmo::new(uuid, Instant::now(), bounding_box, color, shape));
    }
}

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
        stage.clear_old();
    }
}

const MAX_AGE: Duration = Duration::from_secs(4);

pub type Coord = (f32, f32);
pub type Size = (f32, f32);

#[derive(Copy, Clone)]
pub enum BoundingBox {
    Cornered(Coord, Coord),
    Sized(Origin, Size),
}

impl BoundingBox {
    fn center(&self) -> Coord {
        match self {
            BoundingBox::Cornered(p1, p2) => ((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0),
            BoundingBox::Sized(origin, size) => {
                let (x, y) = match origin {
                    Origin::TopLeft(coord) => (coord.0 + size.0 / 2.0, coord.1 + size.1 / 2.0),
                    Origin::TopRight(coord) => (coord.0 - size.0 / 2.0, coord.1 + size.1 / 2.0),
                    Origin::BottomLeft(coord) => (coord.0 + size.0 / 2.0, coord.1 - size.1 / 2.0),
                    Origin::BottomRight(coord) => (coord.0 - size.0 / 2.0, coord.1 - size.1 / 2.0),
                    Origin::Center(coord) => *coord,
                };
                (x, y)
            }
        }
    }

    fn size(&self) -> Size {
        match self {
            BoundingBox::Cornered(p1, p2) => ((p1.0 - p2.0).abs(), (p1.1 - p2.1).abs()),
            BoundingBox::Sized(_, size) => *size,
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
    Rect,
    Ellipse,
}

pub struct Gizmo {
    uuid: Uuid,
    birth: Instant,
    bounding_box: BoundingBox,
    color: Color,
    shape: Shape,
}

impl Gizmo {
    pub fn new(
        uuid: Uuid,
        birth: Instant,
        bounding_box: BoundingBox,
        color: Color,
        shape: Shape,
    ) -> Self {
        Self {
            uuid,
            birth,
            bounding_box,
            color,
            shape,
        }
    }

    pub fn to_raw(&self) -> GizmoRaw {
        let center = self.bounding_box.center();
        let size = self.bounding_box.size();

        let transform: [[f32; 4]; 4] =
            (Matrix4::from_translation(Vector3::new(center.0, center.1, 0.0))
                * Matrix4::from_nonuniform_scale(size.0, size.1, 0.0))
            .into();

        GizmoRaw {
            transform,
            color: [
                self.color.r as f32,
                self.color.g as f32,
                self.color.b as f32,
                self.age().as_secs_f32() / MAX_AGE.as_secs_f32(),
            ],
        }
    }

    pub fn age(&self) -> Duration {
        Instant::now() - self.birth
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GizmoRaw {
    transform: [[f32; 4]; 4],
    color: [f32; 4],
}

impl GizmoRaw {
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
    gizmos: HashMap<Uuid, Gizmo>,
}

impl GizmoStage {
    pub fn render<'a, 'b: 'a, C: CameraController>(
        &'b mut self,
        render_pass: &mut RenderPass<'a>,
        device: &Device,
        queue: &Queue,
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
        render_pass.set_index_buffer(self.mesh.indices().slice(..), IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.mesh.num_indices(), 0, 0..self.gizmos.len() as _);
    }

    pub fn clear_old(&mut self) {
        self.gizmos.retain(|_, gizmo| gizmo.age() < MAX_AGE);
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

    pub fn add_gizmo(&mut self, gizmo: Gizmo) {
        match gizmo.shape {
            Shape::Rect => self.stages[0].gizmos.insert(gizmo.uuid, gizmo),
            Shape::Ellipse => self.stages[1].gizmos.insert(gizmo.uuid, gizmo),
        };
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
