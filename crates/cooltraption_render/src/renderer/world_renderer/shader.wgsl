// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

// Represents a texture region in the atlas
struct Region {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

@group(2) @binding(0)
var<storage, read> regions: array<Region>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct InstanceInput {
    @location(2) model_matrix_0: vec3<f32>, // We cannot use mat4x3 here sadly
    @location(3) model_matrix_1: vec3<f32>,
    @location(4) model_matrix_2: vec3<f32>,
    @location(5) model_matrix_3: vec3<f32>,
    @location(6) texture_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) region_offset: vec2<i32>,
    @location(2) region_size: vec2<i32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        vec4(instance.model_matrix_0, 0.0),
        vec4(instance.model_matrix_1, 0.0),
        vec4(instance.model_matrix_2, 0.0),
        vec4(instance.model_matrix_3, 1.0),
    );

    var out: VertexOutput;

    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);

    let region = regions[instance.texture_index];
    out.region_offset = vec2<i32>(region.x, region.y);
    out.region_size = vec2<i32>(region.width, region.height);

    return out;
}

// Fragment shader

@group(0) @binding(0)
var atlas_texture: texture_2d<f32>;
@group(0) @binding(1)
var atlas_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var atlas_size = textureDimensions(atlas_texture, 0);
    var region_offset = vec2<f32>(in.region_offset) / vec2<f32>(atlas_size);
    var region_size = vec2<f32>(in.region_size) / vec2<f32>(atlas_size);
    var tex_coords = in.tex_coords * region_size + region_offset;

    return textureSample(atlas_texture, atlas_sampler, tex_coords);
}
