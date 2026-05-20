use std::mem;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphInstance {
    pub position: glam::Vec2,
    pub size: glam::Vec2,
    pub rotation: f32,
    pub uv_min: glam::Vec2,
    pub uv_max: glam::Vec2,
    _padding: [f32; 3],
    pub color: glam::Vec4,
}

impl GlyphInstance {
    pub fn new(
        position: glam::Vec2,
        size: glam::Vec2,
        rotation: f32,
        uvs: (glam::Vec2, glam::Vec2),
        color: glam::Vec4,
    ) -> Self {
        Self {
            position,
            size,
            rotation,
            uv_min: uvs.0,
            uv_max: uvs.1,
            _padding: [0.0, 0.0, 0.0],
            color,
        }
    }

    const ATTRIBS: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
        2 => Float32x2,
        3 => Float32x2,
        4 => Float32,
        5 => Float32x2,
        6 => Float32x2,
        7 => Float32x3,
        8 => Float32x4,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
