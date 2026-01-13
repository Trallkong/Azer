// Vertex

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct Uniforms {
    camera: mat4x4<f32>,
    transform: mat3x3<f32>,
}

@group(0) @binding(0) var<uniforms> uniforms: Uniforms;

@vertex
fn vs_main(
    input: VertexInput,
) -> VertexOutput {
    var output: VertexOutput;
    input.position = uniforms.transform * input.position;
    output.position = uniforms.camera * vec4<f32>(input.position, 1.0);
    output.color = input.color;
}

// Fragment

struct FragmentInput {
    @location(0) color: vec3<f32>,
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(
    input: FragmentInput,
) -> FragmentOutput {
    var output: FragmentOutput;
    output.color = vec4<f32>(input.color, 1.0);
    return output;
}