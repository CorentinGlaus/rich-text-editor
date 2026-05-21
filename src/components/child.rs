use crate::{
    components::{Component, child},
    context::Context,
    event::MouseEvent,
};

pub struct Child {
    pub component: Box<dyn Component>,
    pub position: glam::Vec2,
}

impl Child {
    pub fn new(component: Box<dyn Component>, position: glam::Vec2) -> Self {
        Self {
            component,
            position,
        }
    }

    pub fn propagate_init(&mut self, ctx: &mut Context) {
        self.component.propagate_init(ctx);
    }

    pub fn propagate_render(&mut self, ctx: &Context) {
        self.component.propagate_render(ctx);
    }

    pub fn propagate_mouse_event(&mut self, mouse_event: &MouseEvent) {
        self.component.propagate_mouse_event(mouse_event);
    }

    pub fn propagate_keyboard_event(&mut self) {
        self.component.propagate_keyboard_event();
    }
}
