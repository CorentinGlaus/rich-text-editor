use std::mem::{self, offset_of};

use crate::renderer::helper::attr;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphInstance {
    pub local_position: glam::Vec2,
    pub size: glam::Vec2,
    pub uv_min: glam::Vec2,
    pub uv_max: glam::Vec2,
    pub transform_index: u32,
    _pad: [u32; 3],
    pub color: glam::Vec4,
}

impl GlyphInstance {
    pub fn new(
        local_position: glam::Vec2,
        size: glam::Vec2,
        uvs: (glam::Vec2, glam::Vec2),
        transform_index: u32,
        color: glam::Vec4,
    ) -> Self {
        Self {
            local_position,
            size,
            uv_min: uvs.0,
            uv_max: uvs.1,
            transform_index,
            _pad: [0, 0, 0],
            color,
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBS: [wgpu::VertexAttribute; 6] = [
            attr(
                offset_of!(GlyphInstance, local_position),
                2,
                wgpu::VertexFormat::Float32x2,
            ),
            attr(
                offset_of!(GlyphInstance, size),
                3,
                wgpu::VertexFormat::Float32x2,
            ),
            attr(
                offset_of!(GlyphInstance, uv_min),
                4,
                wgpu::VertexFormat::Float32x2,
            ),
            attr(
                offset_of!(GlyphInstance, uv_max),
                5,
                wgpu::VertexFormat::Float32x2,
            ),
            attr(
                offset_of!(GlyphInstance, transform_index),
                6,
                wgpu::VertexFormat::Uint32,
            ),
            attr(
                offset_of!(GlyphInstance, color),
                7,
                wgpu::VertexFormat::Float32x4,
            ),
        ];

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ATTRIBS,
        }
    }
}
