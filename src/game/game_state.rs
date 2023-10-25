use wasm_bindgen::prelude::*;
use crate::{renderer::Context, util::log};

use console_error_panic_hook;
use std::panic;

use crate::util;

#[wasm_bindgen]
pub struct GameState {
    render_context: Context
}

#[wasm_bindgen]
impl GameState {
    pub fn new(canvas_id: &str) -> GameState {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let render_context = Context::new(canvas_id).unwrap();
        return GameState {
            render_context
        }
    }

    pub fn on_frame(&mut self) {
        log("Frame", util::LogLevel::Warning);
        self.render_context.dispatch_draw();
    }

    pub fn on_click(&mut self, _x: i32, _y: i32) {

    }
}