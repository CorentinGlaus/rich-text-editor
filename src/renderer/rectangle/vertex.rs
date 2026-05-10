#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectangleVertex {
    pub position: [f32; 3],
}

impl RectangleVertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RectangleVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

pub const RECTANGLE_VERTICES: &[RectangleVertex] = &[
    RectangleVertex {
        position: [-0.5, 0.5, 0.0],
    }, // top-left
    RectangleVertex {
        position: [-0.5, -0.5, 0.0],
    }, // bottom-left
    RectangleVertex {
        position: [0.5, -0.5, 0.0],
    }, // bottom-right
    RectangleVertex {
        position: [0.5, 0.5, 0.0],
    }, // top-right
];

#[rustfmt::skip]
pub const RECTANGLE_INDICES: &[u16] = &[
    2, 1, 0, // bottom-left triangle
    3, 2, 0, // top-right triangle
];
