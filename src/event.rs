pub struct MouseEvent {
    pub clicked: bool,
    pub moved: bool,
    pub position: glam::Vec2,
}

impl MouseEvent {
    pub fn new_moved_event(position: glam::Vec2) -> Self {
        Self {
            clicked: false,
            moved: true,
            position,
        }
    }
}
