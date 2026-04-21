mod app;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let event_loop = EventLoop::new().expect("Error when creating Event Loop");
    let mut app = App::default();
    event_loop
        .run_app(&mut app)
        .expect("Error in the Event Loop")
}
