use crate::game::particle::Particle;
use crate::math::{Vec2i, Vec2f};
use crate::math::random::random_float;
use crate::game::game_state::ConfigState;

pub struct World {
    pub particles: Vec<Particle>,
    pub config: ConfigState
}

impl World {

    pub fn new(config: ConfigState) -> Self {
        let particles = Vec::new();
        World { particles, config }
    }

    pub fn world_to_gl(bounds: Vec2i, world: Vec2f) -> Vec2f {
        //scale to the range of 0 to 1, then -1 to 1
        let x = world.x / bounds.x as f32;
        let y = world.y / bounds.y as f32;
        let gl_x = x * 2.0 - 1.0;
        let gl_y = y * 2.0 - 1.0;
        Vec2f::new(gl_x, gl_y)
    }

    pub fn create_particle(&mut self) {
        let random_position = Vec2f{x: random_float(0., self.config.bounds.x as f32),y: random_float(0., self.config.bounds.y as f32)};
        let position = World::world_to_gl(self.config.bounds,random_position);
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
        self.particles.iter_mut().for_each(|particle| {
            World::apply_gravity(particle, dt, self.config.gravity_vector);
            World::apply_bounds(particle, dt, self.config.bounds, self.config.wall_damping);
            World::apply_velocity(particle, dt);
        });
        World::solve_overlap(&mut self.particles, dt, self.config.repulsion_force, self.config.particle_radius, self.config.collision_damping);
    }

    fn solve_overlap(particles: &mut Vec<Particle>, dt: f32, repulsion_force: f32, radius: f32, collision_damping: f32) {
        for i in 0..particles.len() {
            for j in 0..particles.len() {
                if i != j {
                    let force = World::push_particles(repulsion_force, radius, collision_damping, dt, &particles[i], &particles[j]);
                    particles[i].velocity = particles[i].velocity.add(&force);
                    particles[j].velocity = particles[j].velocity.add(&force.scale(-1.));
                }
            }
        }
    }

    fn push_particles(repulsion_force: f32, radius: f32, collision_damping: f32,  dt: f32, a: &Particle, b: &Particle) -> Vec2f{
        let dx = a.position.x - b.position.x;
        let dy = a.position.y - b.position.y;

        let distance = (dx * dx + dy * dy).sqrt();

        let repulsion_strength = repulsion_force * dt;
        // Check if the particles are overlapping
        if distance < radius * 2.0 {
            // Calculate the repulsion force
            let force_x = repulsion_strength * dx / distance;
            let force_y = repulsion_strength * dy / distance;

            return Vec2f::new(force_x * collision_damping, force_y * collision_damping);
        } else {
            return Vec2f::new(0., 0.);
        }
    }


    fn apply_gravity(particle: &mut Particle, dt: f32, gravity_vector: Vec2f) {
        let velocity = particle.get_velocity();
        let new_velocity = velocity.add(&gravity_vector.scale(dt));
        particle.set_velocity(new_velocity);
    }

    fn apply_velocity(particle: &mut Particle, dt: f32) {
        let velocity = particle.get_velocity();
        let position = particle.get_position();
        let new_position = position.add(&velocity.scale(dt));
        particle.set_position(new_position);
    }

    fn apply_bounds(particle: &mut Particle, dt: f32, bounds: Vec2i, wall_damping: f32) {
        // Predict next position
        let next_x = particle.position.x + particle.velocity.x * dt;
        let next_y = particle.position.y + particle.velocity.y * dt;

        let min = World::world_to_gl(bounds,Vec2f{x: 0., y: 0.});
        let max = World::world_to_gl(bounds, Vec2f{x: bounds.x as f32, y: bounds.y as f32});

        // Check collision with left or right wall
        if next_x <= min.x || next_x >= max.x {
            particle.velocity.x = -particle.velocity.x * wall_damping;
            particle.position.x = if next_x <= min.x { min.x } else { max.x };
        }
        
        // Check collision with top or bottom wall
        if next_y <= min.y || next_y >= max.y {
            particle.velocity.y = -particle.velocity.y * wall_damping;
            particle.position.y = if next_y <= min.y { min.y } else { max.y };
        }
    }
    
    
}