use std::str::FromStr;

use winit::event_loop::EventLoop;

use crate::{
    app::App,
    components::{
        Component,
        button::{Button, ButtonDescriptor},
    },
};

mod app;
mod camera;
mod components;
mod constants;
mod context;
mod event;
mod macros;
mod renderer;
mod texture;
mod theme;
mod vertex;

fn main() {
    run().unwrap();
}

pub fn run() -> anyhow::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();

    let button = Button::new(ButtonDescriptor {
        width: 200.0,
        height: 100.0,
        text: String::from_str("Test").unwrap(),
        color: None,
    });
    app.root
        .add_child(Box::new(button), glam::vec2(200.0, 200.0));

    event_loop.run_app(&mut app)?;
    Ok(())
}
