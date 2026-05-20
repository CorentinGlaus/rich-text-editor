#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphVertex {
    pub position: glam::Vec2,
    pub tex_coords: glam::Vec2,
}

impl GlyphVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GlyphVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const GLYPH_VERTICES: &[GlyphVertex] = &[
    GlyphVertex {
        position: glam::Vec2::new(0.0, 1.0),
        tex_coords: glam::Vec2::new(0.0, 1.0),
    }, // top-left
    GlyphVertex {
        position: glam::Vec2::new(0.0, 0.0),
        tex_coords: glam::Vec2::new(0.0, 0.0),
    }, // bottom-left
    GlyphVertex {
        position: glam::Vec2::new(1.0, 0.0),
        tex_coords: glam::Vec2::new(1.0, 0.0),
    }, // bottom-right
    GlyphVertex {
        position: glam::Vec2::new(1.0, 1.0),
        tex_coords: glam::Vec2::new(1.0, 1.0),
    }, // top-right
];

#[rustfmt::skip]
pub const GLYPH_INDICES: &[u16] = &[
    2, 1, 0, // bottom-left triangle
    3, 2, 0, // top-right triangle
];
