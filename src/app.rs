use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::{
    components::{Component, root::Root},
    context::Context,
    event::MouseEvent,
    renderer::{
        Renderer,
        draw_manager::{DrawManager, LayerId},
        rectangle::instance::RectangleInstance,
        split::RendererSplit,
    },
    theme::Theme,
};

pub struct App {
    renderer: Option<Renderer>,
    pub theme: Theme,
    pub root: Root,
}

impl App {
    pub fn new() -> Self {
        Self {
            renderer: None,
            theme: create_theme(),
            root: Root::new(),
        }
    }
}

impl ApplicationHandler<Renderer> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("My Editor");
        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create window"),
        );

        self.renderer =
            Some(pollster::block_on(Renderer::new(window)).expect("Failed to create state"));

        let Some(mut ctx) = create_context(&mut self.renderer, &self.theme) else {
            return;
        };
        self.root.propagate_init(&mut ctx);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: Renderer) {
        self.renderer = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.renderer {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => self.handle_key(event_loop, &code, key_state.is_pressed()),
            WindowEvent::CursorMoved { position, .. } => {
                self.root
                    .propagate_mouse_event(&MouseEvent::new_moved_event(glam::vec2(
                        position.x as f32,
                        position.y as f32,
                    )));
            }
            _ => {}
        }
    }
}

impl App {
    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: &KeyCode, is_pressed: bool) {
        #[expect(clippy::single_match)]
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }
}

fn create_theme() -> Theme {
    Theme {
        background_color: glam::Vec4::new(1.0, 1.0, 1.0, 1.0),
        text_color: glam::Vec4::new(0.0, 0.0, 0.0, 1.0),
        primary_color: glam::Vec4::new(0.71765, 0.72549, 0.74510, 1.0),
        // primary_color: glam::Vec4::new(0.0, 0.0, 0.0, 1.0),
    }
}

fn create_context<'t, 'p>(
    renderer: &'p mut Option<Renderer>,
    theme: &'t Theme,
) -> Option<Context<'t, 'p>> {
    let renderer = renderer.as_mut()?;
    let RendererSplit { painter, .. } = renderer.split();
    Some(Context::new(painter, theme))
}
