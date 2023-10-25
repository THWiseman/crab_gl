#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}


impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4f { x, y, z, w }
    }

    pub fn as_float_array(&self) -> [f32; 4] {
        [self.x,self.y,self.z,self.w]
    }
}
