use crate::{
    components::Component,
    context::Context,
    event::MouseEvent,
    renderer::{draw_manager::LayerId, rectangle::instance::RectangleInstance},
    theme,
};

pub struct Button {
    desc: ButtonDescriptor,
}

impl Component for Button {
    fn init(&mut self, ctx: &mut Context) {
        let background = RectangleInstance::new(
            ctx.absolute_position(glam::vec2(0.0, 0.0)),
            glam::vec2(self.desc.width, self.desc.height),
            0.0,
            self.background_color(ctx),
        );
        ctx.painter.create_rect(background, LayerId::CONTENT_LAYER);

        let mut text = ctx.painter.create_text(
            &self.desc.text,
            ctx.absolute_position(glam::vec2(0.0, 0.0)),
            (Some(self.desc.width), Some(self.desc.height)),
            LayerId::CONTENT_LAYER,
            ctx.theme.text_color,
        );
        let text_pos_height_translation = (self.desc.height / 2.0) - (text.height / 2.0);
        let text_pos_width_translation = (self.desc.width / 2.0) - (text.width / 2.0);
        ctx.painter.translate_text(
            text.transform_index,
            glam::vec2(text_pos_width_translation, text_pos_height_translation),
        );
    }

    fn mouse_event(&mut self, mouse_event: &MouseEvent) {}

    fn keyboard_event(&mut self) {}

    fn render(&mut self, ctx: &crate::context::Context) {}
}

impl Button {
    pub fn new(desc: ButtonDescriptor) -> Self {
        Self { desc }
    }

    fn background_color(&self, ctx: &Context) -> glam::Vec4 {
        if let Some(color) = self.desc.color {
            return color;
        }
        ctx.theme.primary_color
    }
}

pub struct ButtonDescriptor {
    pub width: f32,
    pub height: f32,
    pub text: String,
    pub color: Option<glam::Vec4>,
}
