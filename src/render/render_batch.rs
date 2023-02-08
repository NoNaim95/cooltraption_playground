use crate::render::instance::Instance;
use guillotiere::AtlasAllocator;
use wgpu::{BindGroup, Buffer, RenderPipeline};

pub struct RenderBatch {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    diffuse_bind_group: BindGroup,
    instances: Vec<Instance>,
    instance_buffer: Buffer,
    texture_atlas: AtlasAllocator,
}

impl RenderBatch {
    pub fn new(
        render_pipeline: RenderPipeline,
        vertex_buffer: Buffer,
        index_buffer: Buffer,
        num_indices: u32,
        diffuse_bind_group: BindGroup,
        instances: Vec<Instance>,
        instance_buffer: Buffer,
        texture_atlas: AtlasAllocator,
    ) -> Self {
        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            instances,
            instance_buffer,
            texture_atlas,
        }
    }
}
