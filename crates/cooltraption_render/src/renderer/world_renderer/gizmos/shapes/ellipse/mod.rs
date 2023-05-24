use crate::world_renderer::gizmos::shapes::vertex::Vertex;
use crate::world_renderer::gizmos::*;
use cgmath::{Matrix4, Point2, Vector2, Vector3};
use uuid::Uuid;
use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, Face, FragmentState, FrontFace,
    MultisampleState, PolygonMode, PrimitiveState, PrimitiveTopology,
};

pub struct Ellipse {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

impl Ellipse {
    pub fn to_raw(&self, age: Age) -> ShapeRaw {
        let transform: [[f32; 4]; 4] =
            (Matrix4::from_translation(Vector3::new(
                self.x + self.width / 2.0,
                self.y + self.height / 2.0,
                0.0,
            )) * Matrix4::from_nonuniform_scale(self.width, self.height, 0.0))
            .into();

        ShapeRaw {
            transform,
            color: [
                self.color.r as f32,
                self.color.g as f32,
                self.color.b as f32,
                age,
            ],
        }
    }
}

pub fn ellipse(uuid: Uuid, bounding_box: BoundingBox) {
    let top_left = bounding_box.top_left();
    let size = bounding_box.size();

    if let Some(gizmos) = GIZMOS.lock().expect("gizmo mutex").as_mut() {
        gizmos.add_ellipse(
            uuid,
            Ellipse {
                x: top_left.0,
                y: top_left.1,
                width: size.0,
                height: size.1,
                color: Color::BLUE,
            },
        );
    }
}

pub fn create_pipeline(
    device: &Device,
    format: &TextureFormat,
    camera_bgl: &BindGroupLayout,
) -> RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Ellipse Gizmo Render Pipeline Layout"),
        bind_group_layouts: &[camera_bgl],
        push_constant_ranges: &[],
    });

    let shader = device.create_shader_module(include_wgsl!("ellipse.wgsl"));

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Ellipse Gizmo Render Pipeline"),
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
