
use crate::math::Vec2f;

pub struct Particle {
    pub world_position: Vec2f,
    pub velocity: Vec2f,
}

impl Particle {
    pub fn new(world_position: Vec2f, velocity: Vec2f) -> Self {
        Particle { world_position, velocity }
    }

    pub fn get_position(&self) -> Vec2f {
        self.world_position
    }

    pub fn get_velocity(&self) -> Vec2f {
        self.velocity
    }

    pub fn set_position(&mut self, world_position: Vec2f) {
        self.world_position = world_position;
    }

    pub fn set_velocity(&mut self, velocity: Vec2f) {
        self.velocity = velocity;
    }
}