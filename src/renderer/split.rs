use crate::renderer::{painter::Painter, texture_manager::TextureManager};

pub struct RendererSplit<'a> {
    pub painter: Painter<'a>,
    pub textures: &'a mut TextureManager,
}
