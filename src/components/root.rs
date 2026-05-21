use crate::{
    components::{Component, child::Child},
    context::Context,
    event::MouseEvent,
};

pub struct Root {
    children: Vec<Child>,
}

impl Component for Root {
    fn init(&mut self, ctx: &mut Context) {}

    fn render(&mut self, ctx: &Context) {}

    fn mouse_event(&mut self, mouse_event: &MouseEvent) {}

    fn keyboard_event(&mut self) {}

    fn children(&mut self) -> Option<&mut Vec<Child>> {
        Some(&mut self.children)
    }
}

impl Root {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn add_child(&mut self, component: Box<dyn Component>, position: glam::Vec2) {
        self.children.push(Child::new(component, position));
    }
}
