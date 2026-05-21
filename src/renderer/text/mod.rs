mod buffer;
pub mod text_manager;

use crate::renderer::glyph::batch::GlyphHandle;

#[derive(Debug)]
pub struct Text {
    pub glyphs: Vec<GlyphHandle>,
    pub transform_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextTransform {
    pub translation: glam::Vec2,
}
