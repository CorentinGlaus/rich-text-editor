use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::{
    get_image_bytes,
    renderer::{
        Renderer, draw_manager::DrawManager, image::instance::ImageInstance,
        rectangle::instance::RectangleInstance, split::RendererSplit,
    },
    texture_bytes,
};

pub struct App {
    state: Option<Renderer>,
}

impl App {
    pub fn new() -> Self {
        Self { state: None }
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

        self.state =
            Some(pollster::block_on(Renderer::new(window)).expect("Failed to create state"));

        draw_elements(self.state.as_mut().expect("No renderer available"));
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: Renderer) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
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
            } => state.handle_key(event_loop, &code, key_state.is_pressed()),
            _ => {}
        }
    }
}

fn draw_elements(renderer: &mut Renderer) {
    let RendererSplit {
        mut painter,
        textures,
    } = renderer.split();

    let rectangle1 = RectangleInstance::new(
        glam::Vec2::new(300.0, 300.0),
        glam::Vec2::new(300.0, 300.0),
        0.0,
        glam::Vec4::new(1.0, 0.0, 0.0, 0.2),
    );
    let rectangle2 = RectangleInstance::new(
        glam::Vec2::new(200.0, 200.0),
        glam::Vec2::new(300.0, 300.0),
        0.0,
        glam::Vec4::new(0.0, 1.0, 0.0, 0.5),
    );
    // let (rgba, dimensions) = get_image_bytes!(texture_bytes!("house.png"));
    // let house_handle = textures
    //     .add(&rgba, dimensions, 4)
    //     .expect("Error when creating house image");
    // let house_image = ImageInstance::new(
    //     glam::Vec2::new(200.0, 200.0),
    //     glam::Vec2::new(300.0, 300.0),
    //     0.0,
    //     textures.uv(house_handle).expect("House rendered"),
    // );
    // painter.create_image(house_image, DrawManager::CONTENT_LAYER);
    // painter.create_rect(rectangle1, DrawManager::OVERLAY_LAYER);
    let text = painter.create_text(
        "Hello, World!",
        glam::Vec2::new(200.0, 200.0),
        (Some(600.0), None),
        DrawManager::CONTENT_LAYER,
    );

    println!("Text: {:?}", text);
}

pub fn run() -> anyhow::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
