use crate::game::particle::Particle;
use crate::math::{Vec2i, Vec2f};
use crate::math::random::random_float;
use crate::game::game_state::ConfigState;
use std::collections::HashMap;

#[derive()]
pub struct World {
    pub particles: Vec<Particle>,
    pub spatial_partition: HashMap<(i32, i32), Vec<i32>>,
    pub config: ConfigState
}

pub fn world_to_gl(bounds: Vec2i, world: Vec2f) -> Vec2f {
    //scale to the range of 0 to 1, then -1 to 1
    let x = world.x / bounds.x as f32;
    let y = world.y / bounds.y as f32;
    let gl_x = x * 2.0 - 1.0;
    let gl_y = y * 2.0 - 1.0;
    Vec2f::new(gl_x, gl_y)
}

#[allow(unused)]
pub fn gl_to_world(bounds: Vec2i, gl: Vec2f) -> Vec2f {
    //scale to the range of 0 to 1, then -1 to 1
    let x = (gl.x + 1.0) / 2.0;
    let y = (gl.y + 1.0) / 2.0;
    let world_x = x * bounds.x as f32;
    let world_y = y * bounds.y as f32;
    Vec2f::new(world_x, world_y)
}

impl World {
    pub fn new(config: ConfigState) -> Self {
        let particles = Vec::new();
        let spatial_partition = HashMap::new();
        World { particles, spatial_partition, config }
    }

    pub fn create_particle(&mut self) {
        let random_position = Vec2f{x: random_float(0., self.config.bounds.x as f32),y: random_float(0., self.config.bounds.y as f32)};
        let position = random_position;
        let velocity = Vec2f::new(random_float(-30., 100.), 0.);
        let particle = Particle::new(position, velocity);
        self.particles.push(particle);
    }

    pub fn get_spatial_index(bounds: Vec2i, position: Vec2f, partition_radius:f32) -> (i32, i32) {
        let x_partition_size = bounds.x as f32 / partition_radius;
        let y_partition_size = bounds.y as f32 / partition_radius;
        let x = (position.x / x_partition_size).floor() as i32;
        let y = (position.y / y_partition_size).floor() as i32;
        (x, y)
    }

    pub fn get_neighbors(spatial_partition: &HashMap<(i32, i32), Vec<i32>>, spatial_coordinates: (i32, i32)) -> Vec<i32> {
        let top_left = spatial_partition.get(&(spatial_coordinates.0 - 1, spatial_coordinates.1 - 1));
        let top = spatial_partition.get(&(spatial_coordinates.0, spatial_coordinates.1 - 1));
        let top_right = spatial_partition.get(&(spatial_coordinates.0 + 1, spatial_coordinates.1 - 1));
        let left = spatial_partition.get(&(spatial_coordinates.0 - 1, spatial_coordinates.1));
        let right = spatial_partition.get(&(spatial_coordinates.0 + 1, spatial_coordinates.1));
        let bottom_left = spatial_partition.get(&(spatial_coordinates.0 - 1, spatial_coordinates.1 + 1));
        let bottom = spatial_partition.get(&(spatial_coordinates.0, spatial_coordinates.1 + 1));
        let bottom_right = spatial_partition.get(&(spatial_coordinates.0 + 1, spatial_coordinates.1 + 1));
        let middle = spatial_partition.get(&spatial_coordinates);
        
        let neighbors: Vec<Option<&Vec<i32>>> = vec![top_left, top, top_right, left, middle, right, bottom_left, bottom, bottom_right];
        let indexes: Vec<i32>;
        for neighbor in neighbors {
            match neighbor {
                Some(&x) => &indexes.append(&mut x),
                None => ()
            }
        }
        neighbors
    }

    pub fn get_particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    pub fn step(&mut self, dt: f32) {
        let adjusted_dt = dt * self.config.time_multiplier;
        self.particles.iter_mut().for_each(|particle| {
            World::apply_bounds(particle, self.config.bounds, self.config.wall_damping, self.config.particle_radius);
            World::apply_gravity(particle, adjusted_dt, self.config.gravity_vector);
        });
        World::solve_overlap(&mut self.particles, self.config.particle_radius);
        self.particles.iter_mut().for_each(|particle| {
            World::apply_velocity(particle, adjusted_dt, self.config.friction);
        });
    }

    fn distance(vec1: Vec2f, vec2: Vec2f) -> f32 {
        let dx = vec2.x - vec1.x;
        let dy = vec2.y - vec1.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn solve_overlap(particles: &mut Vec<Particle>, radius: f32) {
        for i in 0..particles.len() {

        }
        for i in 0..particles.len() {
            for j in 0..particles.len() {
                if i != j {
                    let particle_a_pos = particles[i].world_position;
                    let particle_a_vel = particles[i].velocity;

                    let particle_b_pos = particles[j].world_position;
                    let particle_b_vel = particles[j].velocity;

                    let distance = World::distance(particle_a_pos, particle_b_pos);
                    if distance < 2.0 * radius {
                        let overlap = (2.0 * radius) - distance;
                        let direction = particle_a_pos.subtract(&particle_b_pos).normalized();
                        particles[i].set_position(particle_a_pos.add(&direction.scale(overlap / 2.0)));
                        particles[i].set_velocity(particle_a_vel.add(&direction.scale(overlap / 2.0)));
                        particles[j].set_position(particle_b_pos.add(&direction.scale(-overlap / 2.0)));
                        particles[j].set_velocity(particle_b_vel.add(&direction.scale(-overlap / 2.0)));
                    }
                }
            }
        }
    }

    fn apply_gravity(particle: &mut Particle, dt: f32, gravity_vector: Vec2f) {
        let velocity = particle.get_velocity();
        let new_velocity = velocity.add(&gravity_vector.scale(dt));
        particle.set_velocity(new_velocity);
    }

    fn apply_velocity(particle: &mut Particle, dt: f32, friction: f32) {
        let velocity = particle.get_velocity().scale(friction);
        let position = particle.get_position();
        let new_position = position.add(&velocity.scale(dt));
        particle.set_position(new_position);
    }

    fn apply_bounds(particle: &mut Particle, bounds: Vec2i, wall_damping: f32, radius: f32) {
        let x = particle.world_position.x;
        let y = particle.world_position.y;

        let min = Vec2f{x: 0. + radius, y: 0. + radius};
        let max = Vec2f{x: bounds.x as f32 - radius, y: bounds.y as f32 - radius};

        // Check collision with left or right wall
        if x <= min.x || x >= max.x {
            particle.velocity.x = -particle.velocity.x * wall_damping;
            particle.world_position.x = if x <= min.x { min.x } else { max.x };
        }
        
        // Check collision with top or bottom wall
        if y <= min.y || y >= max.y {
            particle.velocity.y = -particle.velocity.y * wall_damping;
            particle.world_position.y = if y <= min.y { min.y } else { max.y };
        }
    }
}