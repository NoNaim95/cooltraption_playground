
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct Vertex {
    @location(0) position: vec3<f32>,
};

struct Shape {
    @location(1) transform_0: vec4<f32>,
    @location(2) transform_1: vec4<f32>,
    @location(3) transform_2: vec4<f32>,
    @location(4) transform_3: vec4<f32>,
    @location(5) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: Vertex,
    instance: Shape,
) -> VertexOutput {
    let transform = mat4x4<f32>(
        instance.transform_0,
        instance.transform_1,
        instance.transform_2,
        instance.transform_3,
    );

    var out: VertexOutput;

    out.position = camera.view_proj * transform * vec4<f32>(model.position, 1.0);
    out.tex_coords = model.position.xy;
    out.color = instance.color;

    return out;
}

@fragment
fn fs_main(
    input: VertexOutput,
) -> @location(0) vec4<f32> {
    if (sqrt(pow(input.tex_coords.x, 2.0) + pow(input.tex_coords.y, 2.0)) > 1.0) {
        discard;
    }

    var col = input.color;
    col.a = min(max(-2.0 * col.a + 2.0, 0.0), 1.0) * 0.1;
    return col;
}
