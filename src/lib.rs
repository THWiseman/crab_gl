use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
extern crate js_sys;

mod renderer;
use renderer::{setup_vertices, Context};

mod math;
use math::{Vec3f, Vec4f};

mod shapes;
use shapes::Triangle;

mod util;
use util::log;

mod game;
use game::GameState;

#[wasm_bindgen]
pub fn create_game(canvas_id: &str) -> GameState {
    GameState::new(canvas_id)
}

#[wasm_bindgen]
pub fn draw_triangle(canvas_id: &str, selected_color: Option<Vec<f32>>) -> Result<WebGlRenderingContext, JsValue> {
    let context = Context::new(canvas_id).unwrap();
    let gl: &WebGlRenderingContext = context.get_gl_context();
    let shader_program: &WebGlProgram = context.get_active_shader();

    let top: Vec3f = Vec3f::new(0.0, 1.0, 0.0);
    let bottom_left: Vec3f = Vec3f::new(-1.0, -1.0, 0.0);
    let bottom_right: Vec3f = Vec3f::new(1.0, -1.0, 0.0);
    let triangle: Triangle = Triangle::new(top, bottom_left, bottom_right);
    let vertices: [f32; 9] = triangle.as_float_array();
    setup_vertices(&gl, &vertices, shader_program);

    let color_vec: Vec4f = selected_color.map_or(Vec4f::new(1.0, 0.0, 0.0, 1.0), |c| Vec4f::new(c[0], c[1], c[2], c[3]));
    let color_location = gl
        .get_uniform_location(shader_program, "fragColor")
        .unwrap();
    let floats: [f32; 4] = color_vec.as_float_array().clone();
    gl.uniform4fv_with_f32_array(Some(&color_location), &floats);
    log(&format!("Color: {:?}",floats), util::LogLevel::Warning);

    gl.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );

    Ok(gl.clone())
}