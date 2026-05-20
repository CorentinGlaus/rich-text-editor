pub mod text_manager;

use crate::renderer::glyph::batch::GlyphHandle;

#[derive(Debug)]
pub struct Text {
    pub glyphs: Vec<GlyphHandle>,
}
