use std::mem;

use crate::renderer::texture_manager::TextureHandle;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ImageInstance {
    pub position: glam::Vec2,
    pub scale: glam::Vec2,
    pub rotation: f32,
    pub uv_min: glam::Vec2,
    pub uv_max: glam::Vec2,
}

impl ImageInstance {
    pub fn new(
        position: glam::Vec2,
        scale: glam::Vec2,
        rotation: f32,
        uvs: (glam::Vec2, glam::Vec2),
    ) -> Self {
        Self {
            position,
            scale,
            rotation,
            uv_min: uvs.0,
            uv_max: uvs.1,
        }
    }

    const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
        2 => Float32x2,
        3 => Float32x2,
        4 => Float32,
        5 => Float32x2,
        6 => Float32x2,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
