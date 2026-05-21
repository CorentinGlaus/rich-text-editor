use crate::{renderer::painter::Painter, theme::Theme};

pub struct Context<'t, 'p> {
    pub painter: Painter<'p>,
    pub origin: glam::Vec2,
    pub theme: &'t Theme,
}

impl<'a, 'p> Context<'a, 'p> {
    pub fn new(painter: Painter<'p>, theme: &'a Theme) -> Self {
        Self {
            painter,
            origin: glam::Vec2::ZERO,
            theme,
        }
    }

    pub fn absolute_position(&self, position: glam::Vec2) -> glam::Vec2 {
        self.origin + position
    }
}
