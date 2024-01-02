#import bevy_pbr::{mesh_functions::{get_model_matrix, mesh_position_local_to_clip}, prepass_utils::prepass_depth}

struct CustomMaterial {
    color: vec4<f32>,
};
@group(2) @binding(0) var<uniform> material: CustomMaterial;

// Vertex

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3f,
    @location(1) normal: vec3f,
};

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) normal: vec3f,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.position = mesh_position_local_to_clip(
        get_model_matrix(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );
    out.normal = vertex.normal;
    return out;
}

// Fragment

@fragment
fn fragment(
    #ifdef MULTISAMPLED
    @builtin(sample_index) sample_index: u32,
    #endif
    in: VertexOutput,
) -> @location(0) vec4f {
    #ifndef MULTISAMPLED
    let sample_index = 0u;
    #endif

    // Light
    let light_dir = normalize(vec3f(-3.0, 6.0, -2.0));
    let light_value = dot(in.normal, light_dir) * 0.7 + 0.5;
    let shaded_color = material.color * light_value;

    // Outline
    #ifdef DEPTH_PREPASS
    let depth = prepass_depth(in.position, sample_index);
    let offset = 3.0;
    let delta = 0.0001;

    let depth_right = prepass_depth(in.position + vec4f(offset, 0.0, 0.0, 0.0), sample_index);
    let depth_left = prepass_depth(in.position + vec4f(-offset, 0.0, 0.0, 0.0), sample_index);
    let depth_top = prepass_depth(in.position + vec4f(0.0, offset, 0.0, 0.0), sample_index);
    let depth_bottom = prepass_depth(in.position + vec4f(0.0, -offset, 0.0, 0.0), sample_index);

    let is_outline = abs(depth_right - depth) > delta || abs(depth_left - depth) > delta || abs(depth_top - depth) > delta || abs(depth_bottom - depth) > delta;
    #else
    let is_outline = false;
    #endif

    if is_outline {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }

    return shaded_color;
}
