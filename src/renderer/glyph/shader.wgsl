struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera_uniform: CameraUniform;

struct TextTransform {
    translation: vec2<f32>,
};
@group(2) @binding(0)
var<storage, read> text_transforms: array<TextTransform>;

struct InstanceInput {
    @location(2) position: vec2<f32>,
    @location(3) size: vec2<f32>,
    @location(4) uv_min: vec2<f32>,
    @location(5) uv_max: vec2<f32>,
    @location(6) transform_index: u32,
    @location(7) color: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    let text_tranform = text_transforms[instance.transform_index];

    let scaled = vec3(vertex.position, 0.0) * vec3<f32>(instance.size, 1.0);
    let local_pos = vec3(instance.position, 0.0) + scaled;
    let world_pos = vec3(text_tranform.translation, 0.0) + local_pos;

    var out: VertexOutput;
    out.tex_coords = mix(instance.uv_min, instance.uv_max, vertex.tex_coords);
    out.clip_position = camera_uniform.view_proj * vec4<f32>(world_pos, 1.0);
    out.color = instance.color;
    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = textureSample(texture, texture_sampler, in.tex_coords).r;
    return vec4<f32>(1.0, 1.0, 1.0, alpha) * in.color;
}
