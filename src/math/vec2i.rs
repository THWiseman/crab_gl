#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32
}

impl Vec2i {
    fn new(x: i32, y: i32) -> Self {
        Vec2i { x, y }
    }
}