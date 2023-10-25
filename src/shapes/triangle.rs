use crate::Vec3f;

pub struct Triangle {
    a: Vec3f,
    b: Vec3f,
    c: Vec3f,
}

impl Triangle {
    pub fn new(a: Vec3f, b: Vec3f, c: Vec3f) -> Self {
        Triangle { a, b, c }
    }

    pub fn as_float_array(&self) -> [f32; 9] {
        [
            self.a.x, self.a.y, self.a.z,
            self.b.x, self.b.y, self.b.z,
            self.c.x, self.c.y, self.c.z
        ]
    }
}