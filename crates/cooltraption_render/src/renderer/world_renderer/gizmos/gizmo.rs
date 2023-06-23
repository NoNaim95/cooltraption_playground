use cgmath::{Matrix4, Vector3};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub const MAX_AGE: Duration = Duration::from_secs(4);

pub type Coord = (f32, f32);
pub type Size = (f32, f32);

/// Bounding box of a gizmo, specifies the position and size of the gizmo.
#[derive(Copy, Clone)]
pub enum BoundingBox {
    Cornered(Coord, Coord), // Specify using corners
    Sized(Origin, Size),    // Specify using origin and size
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
    pub const PURPLE: Self = Self::new(0.6, 0.2, 0.8);
    pub const ORANGE: Self = Self::new(1.0, 0.6, 0.0);
    pub const TEAL: Self = Self::new(0.0, 0.5, 0.5);
    pub const ROSE: Self = Self::new(0.91, 0.31, 0.47);
    pub const PINK: Self = Self::new(1.0, 0.4, 0.7);
    pub const LIME: Self = Self::new(0.7, 1.0, 0.0);
    pub const INDIGO: Self = Self::new(0.4, 0.0, 0.8);
    pub const GOLD: Self = Self::new(1.0, 0.8, 0.0);
    pub const AQUA: Self = Self::new(0.0, 0.8, 0.8);
    pub const CRIMSON: Self = Self::new(0.86, 0.08, 0.24);
    pub const EMERALD: Self = Self::new(0.31, 0.78, 0.47);
    pub const SAPPHIRE: Self = Self::new(0.06, 0.32, 0.73);
    pub const CHARCOAL: Self = Self::new(0.21, 0.27, 0.31);
    pub const IVORY: Self = Self::new(0.99, 0.99, 0.94);
    pub const VIOLET: Self = Self::new(0.54, 0.17, 0.89);
    pub const AMBER: Self = Self::new(1.0, 0.75, 0.0);
    pub const TURQUOISE: Self = Self::new(0.19, 0.84, 0.78);
    pub const PLUM: Self = Self::new(0.56, 0.27, 0.52);
    pub const MUSTARD: Self = Self::new(0.74, 0.64, 0.0);
    pub const AZURE: Self = Self::new(0.0, 0.5, 1.0);
    pub const RUBY: Self = Self::new(0.88, 0.07, 0.37);
    pub const CORAL: Self = Self::new(1.0, 0.5, 0.31);
    pub const OCEAN: Self = Self::new(0.0, 0.47, 0.75);
    pub const STEEL: Self = Self::new(0.27, 0.51, 0.71);

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

/// The different shapes a gizmo can have, used to determine the gizmo stage in which the gizmo should be rendered
pub enum Shape {
    Rect,
    Ellipse,
}

pub struct Gizmo {
    uuid: Uuid, // UUID used to uniquely identify the gizmo across frames
    birth: Instant,
    bounding_box: BoundingBox,
    color: Color,
    shape: Shape,
}

impl Gizmo {
    /// Create a new gizmo manually, one should use macros instead
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
