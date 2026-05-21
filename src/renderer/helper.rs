pub const fn attr(offset: usize, loc: u32, format: wgpu::VertexFormat) -> wgpu::VertexAttribute {
    wgpu::VertexAttribute {
        offset: offset as u64,
        shader_location: loc,
        format,
    }
}
