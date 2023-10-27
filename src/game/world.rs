use crate::game::particle::Particle;
use crate::math::{Vec2i, Vec2f};
use crate::math::random::random_float;
use crate::game::game_state::ConfigState;
use std::collections::HashMap;
use crate::util::log::logger::*;

#[derive()]
pub struct World {
    pub particles: Vec<Particle>,
    pub spatial_partition: HashMap<(i32, i32), Vec<i32>>, //key is (row, column), value is a list of particle indexes that live in that partition
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
        let spatial_partition = World::new_spatial_partition_map(config.bounds, config.spatial_partition_size, config.max_particles);
        World { particles, spatial_partition, config }
    }

    pub fn create_particle(&mut self) {
        let random_position = Vec2f{x: random_float(0., self.config.bounds.x as f32),y: random_float(0., self.config.bounds.y as f32)};
        let spatial_partition = World::get_spatial_index(random_position, self.config.bounds, self.config.spatial_partition_size);
        let position = random_position;
        let velocity = Vec2f::new(random_float(-10., 100.), 0.);
        let particle = Particle::new(position, velocity, spatial_partition);
        self.particles.push(particle);
    }

    pub fn get_spatial_index(position: Vec2f, bounds: Vec2i, partition_radius:f32) -> (i32, i32) {
        let x_partition_size = bounds.x as f32 / partition_radius;
        let y_partition_size = bounds.y as f32 / partition_radius;
        let x = (position.x / x_partition_size).floor() as i32;
        let y = (position.y / y_partition_size).floor() as i32;
        (x, y)
    }

    fn new_spatial_partition_map(bounds: Vec2i, partition_radius: f32, max_particles: usize) -> HashMap<(i32, i32), Vec<i32>> {
        let num_x_partitions =(bounds.x as f32 / partition_radius).round() as i32;
        let num_y_partitions = (bounds.y as f32 / partition_radius).round() as i32;
        let mut map = HashMap::new();
        // For each row and column, create a new empty vector.
        for i in 0..num_x_partitions {
            for j in 0..num_y_partitions {
                let key = (i, j);
                let mut value = Vec::new();
                value.reserve(max_particles);
                map.insert(key, value);
            }
        }
        return map;
    }

    fn clear_spatial_partitions(spatial_partition: &mut HashMap<(i32, i32), Vec<i32>>, bounds: Vec2i, partition_radius: f32) {
        let num_x_partitions =(bounds.x as f32 / partition_radius).round() as i32;
        let num_y_partitions = (bounds.y as f32 / partition_radius).round() as i32;
        for i in 0..num_x_partitions {
            for j in 0..num_y_partitions {
                let key = (i, j);
                let partition_vec = spatial_partition.get_mut(&key);
                match partition_vec {
                    Some(vec) => vec.clear(),
                    None => ()
                }
            }
        }
    }

    pub fn get_neighbors(spatial_partition: &HashMap<(i32, i32), Vec<i32>>, spatial_coordinates: (i32, i32), particle_index: i32) -> Vec<i32> {
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
        let mut indexes: Vec<i32> = Vec::new();
        neighbors.iter().for_each(|neighbor_opt|  {
            if neighbor_opt.is_some() {
                let neighbor_list = neighbor_opt.unwrap();
                neighbor_list.iter().for_each(|neighbor_index| {
                    if *neighbor_index != particle_index {
                        indexes.push(neighbor_index.clone());
                    }
                });
            }
        });
        return indexes;
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

        World::solve_overlap(&mut self.particles, &self.spatial_partition,  self.config.particle_radius);
        World::clear_spatial_partitions(&mut self.spatial_partition, self.config.bounds, self.config.spatial_partition_size);

        let num_particles = self.particles.len();
        for i in 0..num_particles {
            let particle = &mut self.particles[i];
            let particle_index = i as i32;
            World::apply_velocity(&mut self.spatial_partition, particle, particle_index, adjusted_dt, self.config.bounds, self.config.spatial_partition_size, self.config.friction);
        }
    }

    fn distance(vec1: Vec2f, vec2: Vec2f) -> f32 {
        let dx = vec2.x - vec1.x;
        let dy = vec2.y - vec1.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn solve_overlap(particles: &mut Vec<Particle>, spatial_partition: &HashMap<(i32, i32), Vec<i32>>, particle_radius: f32) {
        for a_index in 0..particles.len() {
            let particle_a_pos = particles[a_index].world_position;
            let particle_a_vel = particles[a_index].velocity;
            let particle_a_spatial_coordinates = particles[a_index].current_spatial_partition;

            let neighbors = World::get_neighbors(spatial_partition, particle_a_spatial_coordinates, a_index as i32);
            for neighbor in neighbors {
                let b_index: usize = neighbor as usize;
                let particle_b_pos = particles[b_index].world_position;
                let particle_b_vel = particles[b_index].velocity;

                let distance = World::distance(particle_a_pos, particle_b_pos);

                if distance < 2.0 * particle_radius {
                    let overlap = (2.0 * particle_radius) - distance;
                    let direction = particle_a_pos.subtract(&particle_b_pos).normalized();

                    particles[a_index].world_position = particle_a_pos.add(&direction.scale(overlap / 2.0));
                    particles[a_index].velocity = particle_a_vel.add(&direction.scale(overlap / 2.0));

                    particles[b_index].world_position = particle_b_pos.add(&direction.scale(-overlap / 2.0));
                    particles[b_index].velocity = particle_b_vel.add(&direction.scale(-overlap / 2.0));
                }
            }
        }
    }

    fn apply_gravity(particle: &mut Particle, dt: f32, gravity_vector: Vec2f) {
        let new_velocity = particle.velocity.add(&gravity_vector.scale(dt));
        particle.velocity = new_velocity;
    }

    // update the position and spatial partition of the particle, based on its current velocity. Also apply some friction.
    fn apply_velocity(spatial_partition: &mut HashMap<(i32, i32), Vec<i32>>,  particle: &mut Particle, particle_index: i32,  dt: f32, bounds: Vec2i, partition_radius: f32, friction: f32) {

        let current_velocity = particle.velocity;
        let current_position = particle.world_position;

        let new_position = current_position.add(&current_velocity.scale(dt));
        let new_velocity = current_velocity.scale(friction);
        let new_partition = World::get_spatial_index(new_position, bounds, partition_radius);

        particle.world_position = new_position;
        particle.velocity = new_velocity;
        particle.current_spatial_partition = new_partition;
        spatial_partition.entry(new_partition).or_insert(Vec::new()).push(particle_index);
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