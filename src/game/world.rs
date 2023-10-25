use crate::game::particle::Particle;
use crate::math::{Vec2i, Vec2f};
use crate::math::random::random_float;

pub struct World {
    particles: Vec<Particle>,
    bounds: Vec2i
}

impl World{
    const CANVAS_BOUNDS: Vec2i = Vec2i{ x: 800, y: 800 };
    const GRAVITY_VECTOR: Vec2f = Vec2f{ x: 0.0, y: -9.8 };

    pub fn new() -> Self {
        let particles = Vec::new();
        let bounds = World::CANVAS_BOUNDS;
        World { particles, bounds }
    }

    pub fn create_particle(&mut self) {
        let position = Vec2f::new(random_float(-1., 1.), random_float(-1., 1.));
        let velocity = Vec2f::new(0., 0.);
        let particle = Particle::new(position, velocity);
        self.particles.push(particle);
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn get_particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    pub fn step(&mut self, dt: f32) {
        for particle in self.particles.iter_mut() {
            World::apply_gravity(particle, dt);
            World::apply_velocity(particle, dt);
        }
    }

    fn apply_gravity(particle: &mut Particle, dt: f32) {
        let gravity = World::GRAVITY_VECTOR;
        let velocity = particle.get_velocity();
        let new_velocity = velocity.add(&gravity.scale(dt));
        particle.set_velocity(new_velocity);
    }

    fn apply_velocity(particle: &mut Particle, dt: f32) {
        let velocity = particle.get_velocity();
        let position = particle.get_position();
        let new_position = position.add(&velocity.scale(dt));
        particle.set_position(new_position);
    }
}