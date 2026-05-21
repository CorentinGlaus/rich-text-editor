use crate::renderer::{
    draw_manager::LayerId,
    glyph::batch::GlyphBatch,
    image::{
        batch::{ImageBatch, ImageHandle},
        instance::ImageInstance,
    },
    rectangle::{
        batch::{RectangleBatch, RectangleHandle},
        instance::RectangleInstance,
    },
    text::{Text, text_manager::TextManager},
};

pub struct Painter<'a> {
    rects: &'a mut RectangleBatch,
    images: &'a mut ImageBatch,
    glyphs: &'a mut GlyphBatch,
    texts: &'a mut TextManager,
}

impl<'a> Painter<'a> {
    pub fn new(
        rects: &'a mut RectangleBatch,
        images: &'a mut ImageBatch,
        glyphs: &'a mut GlyphBatch,
        texts: &'a mut TextManager,
    ) -> Self {
        Self {
            rects,
            images,
            texts,
            glyphs,
        }
    }

    pub fn create_rect(&mut self, rectangle: RectangleInstance, layer: LayerId) -> RectangleHandle {
        self.rects.create(rectangle, layer)
    }

    pub fn create_image(&mut self, image: ImageInstance, layer: LayerId) -> ImageHandle {
        self.images.create(image, layer)
    }

    pub fn create_text(
        &mut self,
        text: &str,
        position: glam::Vec2,
        size: (Option<f32>, Option<f32>),
        layer: LayerId,
        color: glam::Vec4,
    ) -> Text {
        self.texts
            .create_text(text, position, size, self.glyphs, layer, color)
    }
}
