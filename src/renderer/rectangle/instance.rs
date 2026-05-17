use std::mem;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectangleInstance {
    pub position: glam::Vec3,
    pub scale: glam::Vec2,
    pub rotation: f32,
    _padding: [f32; 2],
    pub color: glam::Vec4,
}

impl RectangleInstance {
    pub fn new(position: glam::Vec3, scale: glam::Vec2, rotation: f32, color: glam::Vec4) -> Self {
        Self {
            position,
            scale,
            rotation,
            _padding: [0.0, 0.0],
            color,
        }
    }

    const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
        1 => Float32x3,
        2 => Float32x2,
        3 => Float32,
        4 => Float32x2,
        5 => Float32x4,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RectangleInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
