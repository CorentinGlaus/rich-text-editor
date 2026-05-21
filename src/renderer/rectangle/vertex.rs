#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectangleVertex {
    pub position: glam::Vec2,
}

impl RectangleVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
        0 => Float32x2,
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RectangleVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const RECTANGLE_VERTICES: &[RectangleVertex] = &[
    RectangleVertex {
        position: glam::Vec2::new(0.0, 1.0),
    }, // top-left
    RectangleVertex {
        position: glam::Vec2::new(0.0, 0.0),
    }, // bottom-left
    RectangleVertex {
        position: glam::Vec2::new(1.0, 0.0),
    }, // bottom-right
    RectangleVertex {
        position: glam::Vec2::new(1.0, 1.0),
    }, // top-right
];

#[rustfmt::skip]
pub const RECTANGLE_INDICES: &[u16] = &[
    2, 1, 0, // bottom-left triangle
    3, 2, 0, // top-right triangle
];
