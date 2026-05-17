struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera_uniform: CameraUniform;

struct InstanceInput {
    @location(1) position: vec2<f32>,
    @location(2) scale: vec2<f32>,
    @location(3) rotation: f32,
    @location(4) _padding: vec3<f32>,
    @location(5) color: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    let cos_rotation = cos(instance.rotation);
    let sin_rotation = sin(instance.rotation);

    // TODO: Remove the Vec3 for the calculations, only add at the end.
    let scaled = vec3(vertex.position, 0.0) * vec3<f32>(instance.scale, 1.0);
    let rotated = vec3<f32>(
        scaled.x * cos_rotation - scaled.y * sin_rotation,
        scaled.x * sin_rotation + scaled.y * cos_rotation,
        scaled.z
    );
    let world_pos = vec3(instance.position, 0.0) + rotated;

    var out: VertexOutput;
    out.color = instance.color;
    out.clip_position = camera_uniform.view_proj * vec4<f32>(world_pos, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
