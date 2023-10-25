use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
extern crate js_sys;

mod renderer;
use renderer::{setup_vertices, Context, CircleBuffer};

mod math;
use math::{Vec3f, Vec4f};

mod shapes;
use shapes::Triangle;

mod util;
use util::log;
use util::LogLevel::Warning;

use console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = ANGLEInstancedArrays)]
    type AngleInstancedArrays;

    #[wasm_bindgen(method, getter, js_name = VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE)]
    fn vertex_attrib_array_divisor_angle(this: &AngleInstancedArrays) -> i32;

    #[wasm_bindgen(method, catch, js_name = drawArraysInstancedANGLE)]
    fn draw_arrays_instanced_angle(this: &AngleInstancedArrays, mode: u32, first: i32, count: i32, primcount: i32) -> Result<(), JsValue>;

    // TODO offset should be i64
    #[wasm_bindgen(method, catch, js_name = drawElementsInstancedANGLE)]
    fn draw_elements_instanced_angle(this: &AngleInstancedArrays, mode: u32, count: i32, type_: u32, offset: i32, primcount: i32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = vertexAttribDivisorANGLE)]
    fn vertex_attrib_divisor_angle(this: &AngleInstancedArrays, index: u32, divisor: u32);
}

#[wasm_bindgen]
pub fn draw_circles(canvas_id: &str, selected_color: Option<Vec<f32>>) -> Result<WebGlRenderingContext, JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let context = Context::new(canvas_id).unwrap();
    let gl: &WebGlRenderingContext = context.get_gl_context();
    let shader_program: &WebGlProgram = context.get_active_shader();

    // enable the instanced rendering extension
    gl.get_supported_extensions().map(|ext|
        {
        log(&format!("Extension: {:?}", ext), Warning);
    });
    let extension: js_sys::Object = gl.get_extension("ANGLE_instanced_arrays").expect("Unable to find ANGLE_instanced_arrays").unwrap();
    let ext: AngleInstancedArrays = extension.dyn_into::<AngleInstancedArrays>().expect("Failed to cast extension to AngleInstancedArrays");

    let circle_buffer: CircleBuffer = renderer::CircleBuffer::new(gl, shader_program);

    let color_vec: Vec4f = selected_color.map_or(Vec4f::new(1.0, 0.0, 0.0, 1.0), |c| Vec4f::new(c[0], c[1], c[2], c[3]));
    let color_location = gl
        .get_uniform_location(shader_program, "fragColor")
        .unwrap();
    let floats: [f32; 4] = color_vec.as_float_array().clone();
    gl.uniform4fv_with_f32_array(Some(&color_location), &floats);
    log(&format!("Color: {:?}",floats), util::LogLevel::Warning);

    circle_buffer.render(gl, &ext);
    return Ok(gl.clone());
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