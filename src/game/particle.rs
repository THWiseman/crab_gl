
use crate::math::Vec2f;

pub struct Particle {
    position: Vec2f,
    velocity: Vec2f,
}

impl Particle {
    pub fn new(position: Vec2f, velocity: Vec2f) -> Self {
        Particle { position, velocity }
    }

    pub fn get_position(&self) -> &Vec2f {
        &self.position
    }

    pub fn get_velocity(&self) -> &Vec2f {
        &self.velocity
    }

    pub fn set_position(&mut self, position: Vec2f) {
        self.position = position;
    }

    pub fn set_velocity(&mut self, velocity: Vec2f) {
        self.velocity = velocity;
    }
}