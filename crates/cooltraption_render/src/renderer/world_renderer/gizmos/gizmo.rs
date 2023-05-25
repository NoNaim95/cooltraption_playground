use cgmath::{Matrix4, Vector3};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub const MAX_AGE: Duration = Duration::from_secs(4);

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

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
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
