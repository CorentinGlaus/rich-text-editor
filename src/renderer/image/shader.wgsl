struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera_uniform: CameraUniform;

struct InstanceInput {
    @location(2) position: vec2<f32>,
    @location(3) scale: vec2<f32>,
    @location(4) rotation: f32,
    @location(5) uv_min: vec2<f32>,
    @location(6) uv_max: vec2<f32>,
};

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    let cos_rotation = cos(instance.rotation);
    let sin_rotation = sin(instance.rotation);

    let scaled = vec3(vertex.position, 0.0) * vec3<f32>(instance.scale, 1.0);
    let rotated = vec3<f32>(
        scaled.x * cos_rotation - scaled.y * sin_rotation,
        scaled.x * sin_rotation + scaled.y * cos_rotation,
        scaled.z
    );
    let world_pos = vec3(instance.position, 0.0) + rotated;

    var out: VertexOutput;
    out.tex_coords = mix(instance.uv_min, instance.uv_max, vertex.tex_coords);
    out.clip_position = camera_uniform.view_proj * vec4<f32>(world_pos, 1.0);
    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in.tex_coords);
}
