use crate::world_renderer::gizmos::shapes::vertex::Vertex;
use crate::world_renderer::gizmos::*;
use cgmath::Point2;
use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, Face, FragmentState, FrontFace,
    MultisampleState, PolygonMode, PrimitiveState, PrimitiveTopology,
};

pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

impl Rect {
    pub fn to_raw(&self) -> ShapeRaw {
        ShapeRaw {
            transform: [
                [self.width, 0.0, 0.0, 0.0],
                [0.0, self.height, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [self.x, self.y, 0.0, 1.0],
            ],
            color: [
                self.color.r as f32,
                self.color.g as f32,
                self.color.b as f32,
            ],
        }
    }
}

pub fn rect(p1: Point2<f32>, p2: Point2<f32>) {
    GIZMOS
        .lock()
        .expect("gizmo mutex")
        .as_mut()
        .expect("initialized gizmos")
        .add_rect(Rect {
            x: p1.x,
            y: p1.y,
            width: p2.x - p1.x,
            height: p2.y - p1.y,
            color: Color::RED,
        });
}

pub fn create_pipeline(
    device: &Device,
    format: &TextureFormat,
    camera_bgl: &BindGroupLayout,
) -> RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Rect Gizmo Render Pipeline Layout"),
        bind_group_layouts: &[camera_bgl],
        push_constant_ranges: &[],
    });

    let shader = device.create_shader_module(include_wgsl!("rect.wgsl"));

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Rect Gizmo Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc(), ShapeRaw::desc()],
        },
        fragment: Some(FragmentState {
            module: &shader,
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

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
    },
];

pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];
