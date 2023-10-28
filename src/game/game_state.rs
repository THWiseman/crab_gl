use wasm_bindgen::prelude::*;
use crate::{renderer::Context, util};
use crate::game::world::*;
use console_error_panic_hook;
use std::panic;
use crate::math::{Vec2i, Vec2f};

#[wasm_bindgen]
pub struct GameState {
    render_context: Context,
    physics_simulation: World,
}

pub struct ConfigState {
    pub bounds: Vec2i,
    pub max_particles: usize,
    pub particle_radius: f32,
    pub gravity_vector: Vec2f,
    pub wall_damping: f32,
    pub repulsion_force: f32,
    pub collision_damping: f32,
    pub friction: f32,
    pub time_multiplier: f32,
    pub spatial_partition_size: f32,
}
pub const DEFAULT_BOUNDS: Vec2i = Vec2i{ x: 800, y: 800 };
pub const DEFAULT_MAX_PARTICLES: usize = 1500;
pub const DEFAULT_PARTICLE_RADIUS: f32 = 5.;
pub const DEFAULT_GRAVITY_VECTOR: Vec2f = Vec2f{ x: 0.0, y: -9.8 };
pub const DEFAULT_WALL_DAMPING: f32 = 0.9;
pub const DEFAULT_REPULSION_FORCE: f32 = 10.;
pub const DEFAULT_COLLISION_DAMPING: f32 = 0.9;
pub const DEFAULT_FRICTION: f32 = 0.999;
pub const DEFAULT_TIME_MULTIPLIER: f32 = 3.0;
pub const DEFAULT_SPATIAL_PARTITION_SIZE: f32 = 100.;

impl ConfigState{
    fn new() -> ConfigState{
        let bounds = DEFAULT_BOUNDS;
        let max_particles = DEFAULT_MAX_PARTICLES;
        let particle_radius = DEFAULT_PARTICLE_RADIUS;
        let gravity_vector = DEFAULT_GRAVITY_VECTOR;
        let wall_damping = DEFAULT_WALL_DAMPING;
        let repulsion_force = DEFAULT_REPULSION_FORCE;
        let collision_damping = DEFAULT_COLLISION_DAMPING;
        let friction = DEFAULT_FRICTION;
        let time_multiplier = DEFAULT_TIME_MULTIPLIER;
        let spatial_partition_size = DEFAULT_SPATIAL_PARTITION_SIZE;
        return ConfigState{ bounds, max_particles, particle_radius, gravity_vector, wall_damping, repulsion_force, collision_damping, friction, time_multiplier, spatial_partition_size }
    }
}

#[wasm_bindgen]
impl GameState {
    pub fn new(canvas_id: &str) -> GameState {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let render_context = Context::new(canvas_id).unwrap();

        let mut physics_simulation = World::new(ConfigState::new());
        for _ in 0..DEFAULT_MAX_PARTICLES {
            physics_simulation.create_particle();
        }

        return GameState {
            render_context,
            physics_simulation,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.physics_simulation.step(dt);
    }

    pub fn render(&mut self) {
        self.update_render_state();
        self.render_context.dispatch_draw();
    }

    pub fn on_click(&mut self, _x: i32, _y: i32) {
        let mut physics_simulation = World::new(ConfigState::new());
        for _ in 0..DEFAULT_MAX_PARTICLES {
            physics_simulation.create_particle();
        }
        self.physics_simulation = physics_simulation;
    }

    fn update_render_state(&mut self){
        let particles = self.physics_simulation.get_particles();
        let instances = self.render_context.get_mutable_instances();
        for (i, particle) in particles.iter().enumerate() {
            let position = world_to_gl(self.physics_simulation.config.bounds, particle.world_position);
            instances[i].center_x = position.x;
            instances[i].center_y = position.y;
        }
    }
}