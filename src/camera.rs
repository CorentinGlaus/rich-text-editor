use wgpu::util::DeviceExt;

pub struct Camera {
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn build_view_proj_matrix(&self) -> glam::Mat4 {
        glam::Mat4::orthographic_rh(0.0, self.width as f32, self.height as f32, 0.0, -1.0, 1.0)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_proj_matrix().to_cols_array_2d();
    }
}

pub fn create_camera_buffer(device: &wgpu::Device, camera_uniform: CameraUniform) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

pub fn create_camera_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("Camera Bind Group Layout"),
    })
}

pub fn create_camera_bind_group(
    device: &wgpu::Device,
    camera_buffer: &wgpu::Buffer,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
        label: Some("Camera Bind Group"),
    })
}
