use std::sync::Arc;

use anyhow::bail;
use guillotiere::{
    AllocId, AtlasAllocator,
    euclid::{Box2D, rect},
    point2, size2,
};
use image::GenericImageView;
use slotmap::SlotMap;

use crate::texture::{Texture, TextureFormat};

slotmap::new_key_type! {
    pub struct TextureHandle;
}

struct AtlasRegion {
    rect: guillotiere::Rectangle,
    alloc_id: AllocId,
}

pub struct TextureManager {
    atlas_texture: Texture,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    allocator: AtlasAllocator,
    regions: SlotMap<TextureHandle, AtlasRegion>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    format: TextureFormat,
}

impl TextureManager {
    pub fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        start_size: (u32, u32),
        format: TextureFormat,
    ) -> anyhow::Result<Self> {
        let texture = Texture::empty(
            &device,
            start_size,
            format.clone(),
            wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            Some("Atlas Texture"),
        )?;

        let texture_bind_group_layout = Texture::bind_group_layout(&device);
        let texture_bind_group = texture.create_bind_group(&device, &texture_bind_group_layout);

        let atlas_allocator = AtlasAllocator::new(size2(start_size.0 as i32, start_size.1 as i32));

        Ok(Self {
            atlas_texture: texture,
            bind_group_layout: texture_bind_group_layout,
            bind_group: texture_bind_group,
            allocator: atlas_allocator,
            regions: SlotMap::with_key(),
            device,
            queue,
            format,
        })
    }

    const PAD: u32 = 1;

    pub fn add(&mut self, bytes: &[u8], dimensions: (u32, u32)) -> Option<TextureHandle> {
        let (w, h) = dimensions;
        if w == 0 || h == 0 {
            return None;
        }
        let padded = (w + 2 * Self::PAD, h + 2 * Self::PAD);

        let allocation = loop {
            let allocation = self
                .allocator
                .allocate(size2(padded.0 as i32, padded.1 as i32));
            if let Some(allocation) = allocation {
                let rect = allocation.rectangle;
                self.queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &self.atlas_texture.texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d {
                            x: rect.min.x as u32 + Self::PAD,
                            y: rect.min.y as u32 + Self::PAD,
                            z: 0,
                        },
                        aspect: wgpu::TextureAspect::All,
                    },
                    bytes,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(self.atlas_texture.bytes_per_pixel() * dimensions.0),
                        rows_per_image: Some(dimensions.1),
                    },
                    wgpu::Extent3d {
                        width: dimensions.0,
                        height: dimensions.1,
                        depth_or_array_layers: 1,
                    },
                );
                break allocation;
            }
            let texture_size = self.atlas_texture.texture.size();
            self.grow_atlas((texture_size.width * 2, texture_size.height * 2));
        };

        Some(self.regions.insert(AtlasRegion {
            rect: Box2D::new(
                point2(
                    allocation.rectangle.min.x + Self::PAD as i32,
                    allocation.rectangle.min.y + Self::PAD as i32,
                ),
                point2(
                    allocation.rectangle.max.x - Self::PAD as i32,
                    allocation.rectangle.max.y - Self::PAD as i32,
                ),
            ),
            alloc_id: allocation.id,
        }))
    }

    pub fn remove(&mut self, handle: TextureHandle) {
        if let Some(allocation) = self.regions.remove(handle) {
            self.allocator.deallocate(allocation.alloc_id);
        }
    }

    pub fn uv(&self, handle: TextureHandle) -> Option<(glam::Vec2, glam::Vec2)> {
        self.regions.get(handle).map(|r| {
            let atlas_size = self.atlas_texture.texture.size();
            let uv_min = glam::Vec2::new(
                r.rect.min.x as f32 / atlas_size.width as f32,
                r.rect.min.y as f32 / atlas_size.height as f32,
            );
            let uv_max = glam::Vec2::new(
                r.rect.max.x as f32 / atlas_size.width as f32,
                r.rect.max.y as f32 / atlas_size.height as f32,
            );
            (uv_min, uv_max)
        })
    }

    pub fn size(&self, handle: TextureHandle) -> Option<(glam::Vec2)> {
        self.regions.get(handle).map(|r| {
            glam::vec2(
                (r.rect.max.x - r.rect.min.x) as f32,
                (r.rect.max.y - r.rect.min.y) as f32,
            )
        })
    }

    fn grow_atlas(&mut self, new_size: (u32, u32)) {
        let max = self.device.limits().max_texture_dimension_2d;
        assert!(
            new_size.0 <= max && new_size.1 <= max,
            "atlas exceeds device limit"
        );
        let new_texture = Texture::empty(
            &self.device,
            new_size,
            self.format.clone(),
            wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            Some("Atlas Texture"),
        )
        .expect("Error when creating texture and growing atlas");
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.atlas_texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyTextureInfo {
                texture: &new_texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            self.atlas_texture.texture.size(),
        );
        self.queue.submit(std::iter::once(encoder.finish()));

        self.bind_group = new_texture.create_bind_group(&self.device, &self.bind_group_layout);
        self.atlas_texture = new_texture;
        self.allocator
            .grow(size2(new_size.0 as i32, new_size.1 as i32));
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
