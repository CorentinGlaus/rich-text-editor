use anyhow::bail;

use crate::renderer::{
    image::batch::ImageBatch, rectangle::batch::RectangleBatch, texture_manager::TextureManager,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct LayerId(i32);

pub struct DrawManager {
    rectangle_batch: RectangleBatch,
    image_batch: ImageBatch,
    layers: Vec<LayerId>,
}

impl DrawManager {
    pub const BACKGROUND_LAYER: LayerId = LayerId(0);
    pub const CONTENT_LAYER: LayerId = LayerId(1);
    pub const OVERLAY_LAYER: LayerId = LayerId(2);

    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        texture_manager: &TextureManager,
    ) -> anyhow::Result<Self> {
        let layers = vec![
            Self::BACKGROUND_LAYER,
            Self::CONTENT_LAYER,
            Self::OVERLAY_LAYER,
        ];
        let rectangle_batch = RectangleBatch::new(device, config, camera_bind_group_layout);
        let image_batch = ImageBatch::new(
            device,
            config,
            &[
                Some(camera_bind_group_layout),
                Some(texture_manager.bind_group_layout()),
            ],
        );

        Ok(Self {
            rectangle_batch,
            image_batch,
            layers,
        })
    }

    pub fn add_layer(&mut self, layer: LayerId) -> anyhow::Result<()> {
        if self.layers.contains(&layer) {
            bail!("Layer Id {layer:?} already exists");
        }
        self.layers.push(layer);
        Ok(())
    }

    pub fn remove_layer(&mut self, layer: LayerId) -> anyhow::Result<()> {
        if let Some(pos) = self.layers.iter().position(|x| *x == layer) {
            self.layers.remove(pos);
            return Ok(());
        }
        bail!("No layer found for Layer Id {layer:?}");
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass,
        texture_bind_group: &wgpu::BindGroup,
    ) {
        self.rectangle_batch
            .update_buffer(device, queue, &self.layers);
        self.image_batch.update_buffer(device, queue, &self.layers);
        for layer in self.layers.iter() {
            self.rectangle_batch.draw_layer(render_pass, layer);
            self.image_batch
                .draw_layer(render_pass, texture_bind_group, layer);
        }
    }

    pub fn rectangle_batch(&mut self) -> &mut RectangleBatch {
        &mut self.rectangle_batch
    }

    pub fn image_batch(&mut self) -> &mut ImageBatch {
        &mut self.image_batch
    }
}
