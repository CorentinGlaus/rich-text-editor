use std::{collections::HashMap, sync::Arc};

use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, SwashCache};

use crate::renderer::{
    draw_manager::LayerId,
    glyph::{
        batch::{GlyphBatch, GlyphHandle},
        instance::GlyphInstance,
    },
    text::{Text, TextTransform, buffer::TextTransformBuffer},
    texture_manager::{TextureHandle, TextureManager},
};

#[derive(Clone, Copy)]
struct CachedGlyph {
    handle: TextureHandle,
    placement_left: i32,
    placement_top: i32,
}

pub struct TextManager {
    font_system: FontSystem,
    swash_cache: SwashCache,
    pub(crate) glyph_atlas: TextureManager,
    glyph_cache: HashMap<cosmic_text::CacheKey, CachedGlyph>,
    buffer: TextTransformBuffer,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl TextManager {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let glyph_manager = TextureManager::new(
            device.clone(),
            queue.clone(),
            (2048, 2048),
            crate::texture::TextureFormat::R8,
        )
        .expect("Error when creating texture manager");
        let glyph_cache = HashMap::new();

        let buffer = TextTransformBuffer::new(&device);
        Self {
            font_system,
            swash_cache,
            glyph_atlas: glyph_manager,
            glyph_cache,
            buffer,
            device,
            queue,
        }
    }

    pub fn create_text(
        &mut self,
        text: &str,
        position: glam::Vec2,
        size: (Option<f32>, Option<f32>),
        batch: &mut GlyphBatch,
        layer_id: LayerId,
        color: glam::Vec4,
    ) -> Text {
        let metrics = Metrics::new(64.0, 80.0);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        buffer.set_size(size.0, size.1);
        buffer.set_text(
            text,
            &Attrs::new(),
            cosmic_text::Shaping::Advanced,
            Some(cosmic_text::Align::Left),
        );
        buffer.shape_until_scroll(&mut self.font_system, false);

        let mut glyphs: Vec<GlyphHandle> = vec![];
        let text_transform = TextTransform {
            translation: position,
        };
        let transform_index = self.buffer.alloc(text_transform);

        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical = glyph.physical((0.0, run.line_y), 1.0);
                let key = physical.cache_key;

                let mut cached_glyph = self.glyph_cache.get(&key).copied();

                if cached_glyph.is_none() {
                    let image = self
                        .swash_cache
                        .get_image(&mut self.font_system, key)
                        .as_ref()
                        .expect("Rasterization failed");

                    let texture_handle = self
                        .glyph_atlas
                        .add(&image.data, (image.placement.width, image.placement.height));

                    if let Some(texture_handle) = texture_handle {
                        let new_cached_glyph = CachedGlyph {
                            handle: texture_handle,
                            placement_left: image.placement.left,
                            placement_top: image.placement.top,
                        };

                        self.glyph_cache.insert(key, new_cached_glyph);
                        cached_glyph = Some(new_cached_glyph);
                    }
                }

                if let Some(cached_glyph) = cached_glyph {
                    let uvs = self
                        .glyph_atlas
                        .uv(cached_glyph.handle)
                        .expect("No UVs found for the glyph");
                    let image_size = self
                        .glyph_atlas
                        .size(cached_glyph.handle)
                        .expect("No size found for the glyph");
                    let draw_x = physical.x as f32 + cached_glyph.placement_left as f32;
                    let draw_y = physical.y as f32 - cached_glyph.placement_top as f32;
                    let glyph_instance =
                        GlyphInstance::new(glam::vec2(draw_x, draw_y), image_size, uvs, 0, color);
                    let glyph_handle = batch.create(glyph_instance, layer_id);
                    glyphs.push(glyph_handle);
                };
            }
        }

        Text {
            glyphs,
            transform_index,
        }
    }

    pub fn bind_buffer(&mut self, render_pass: &mut wgpu::RenderPass) {
        self.update_buffer();
        self.buffer.bind(render_pass);
    }

    pub fn update_buffer(&mut self) {
        self.buffer.update_buffer(&self.device, &self.queue);
    }

    pub fn text_buffer_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        self.buffer.bind_group_layout()
    }
}
