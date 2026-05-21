pub mod button;
pub mod child;
pub mod root;

use crate::{components::child::Child, context::Context, event::MouseEvent};

pub trait Component {
    fn propagate_init(&mut self, ctx: &mut Context) {
        if let Some(children) = self.children() {
            let saved = ctx.origin;
            for child in children {
                ctx.origin = saved + child.position;
                child.propagate_init(ctx);
            }
            ctx.origin = saved;
        };
        self.init(ctx);
    }
    fn init(&mut self, ctx: &mut Context);

    fn propagate_render(&mut self, ctx: &Context) {
        if let Some(children) = self.children() {
            for child in children {
                child.propagate_render(ctx);
            }
        };
        self.render(ctx);
    }
    fn render(&mut self, ctx: &Context);

    fn propagate_mouse_event(&mut self, mouse_event: &MouseEvent) {
        if let Some(children) = self.children() {
            for child in children {
                child.propagate_mouse_event(mouse_event);
            }
        };
        self.mouse_event(mouse_event);
    }
    fn mouse_event(&mut self, mouse_event: &MouseEvent);

    fn propagate_keyboard_event(&mut self) {
        if let Some(children) = self.children() {
            for child in children {
                child.propagate_keyboard_event();
            }
        };
        self.keyboard_event();
    }
    fn keyboard_event(&mut self);

    fn children(&mut self) -> Option<&mut Vec<Child>> {
        None
    }
}
