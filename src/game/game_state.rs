use wasm_bindgen::prelude::*;
use crate::{renderer::Context, util::log};
use crate::game::world::World;
use console_error_panic_hook;
use std::panic;
use std::time::{Duration, SystemTime};

use crate::util;

#[wasm_bindgen]
pub struct GameState {
    render_context: Context,
    physics_simulation: World,
    earlier: SystemTime
}

pub const MAX_PARTICLES: usize = 10;

#[wasm_bindgen]
impl GameState {
    pub fn new(canvas_id: &str) -> GameState {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let render_context = Context::new(canvas_id).unwrap();

        let mut physics_simulation = World::new();
        for _ in 0..MAX_PARTICLES {
            physics_simulation.create_particle();
        }

        let earlier = SystemTime::now();

        return GameState {
            render_context,
            physics_simulation,
            earlier
        }
    }

    pub fn on_frame(&mut self) {
        let now = SystemTime::now();
        let dt = now.duration_since(self.earlier).unwrap().as_secs_f32();
        self.earlier = now;
        
        log("Frame", util::LogLevel::Warning);
        self.physics_simulation.step(dt);
        self.update_render_state();
        self.render_context.dispatch_draw();
    }

    pub fn on_click(&mut self, _x: i32, _y: i32) {

    }

    fn update_render_state(&mut self){
        let particles = self.physics_simulation.get_particles();
        let instances = self.render_context.get_mutable_instances();
        for (i, particle) in particles.iter().enumerate() {
            let position = particle.get_position();
            instances[i].center_x = position.x;
            instances[i].center_y = position.y;
        }
    }
}