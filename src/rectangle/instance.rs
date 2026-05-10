pub struct RectangleInstance {
    pub position: glam::Vec3,
    pub angle_z: f32,
    pub scale: glam::Vec3,
    pub color: glam::Vec3,
}

impl RectangleInstance {
    pub fn to_raw(&self) -> RectangleInstanceRaw {
        RectangleInstanceRaw {
            model: (glam::Mat4::from_translation(self.position)
                * glam::Mat4::from_rotation_z(self.angle_z)
                * glam::Mat4::from_scale(self.scale))
            .to_cols_array_2d(),
            color: self.color.to_array(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectangleInstanceRaw {
    model: [[f32; 4]; 4],
    color: [f32; 3],
}

impl RectangleInstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RectangleInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
