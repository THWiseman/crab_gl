
use crate::math::Vec2f;

pub struct Particle {
    pub world_position: Vec2f,
    pub velocity: Vec2f,
    pub current_spatial_partition: (i32, i32)
}

impl Particle {
    pub fn new(world_position: Vec2f, velocity: Vec2f, current_spatial_partition: (i32, i32)) -> Self {
        Particle { world_position, velocity, current_spatial_partition }
    }
}