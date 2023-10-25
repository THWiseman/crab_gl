use game::particle::Particle;

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

let random_value = RandomState::new().build_hasher().finish() as usize;
println!("Random: {}", random_value);
struct World {
    particles: Vec<Particle>
}

impl World{
    pub fn new(num_particles: i32) -> Self {
        let mut particles = Vec::new();
        for _ in 0..num_particles {
            particles.push(Particle::new(Vec2f::new(0.0, 0.0), Vec2f::new(0.0, 0.0)));
        }
        World { particles }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn get_particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    pub fn rand() -> usize {
        let mut hasher = RandomState::new().build_hasher();
        hasher.write_i32(0);
        hasher.finish() as usize
    }
}