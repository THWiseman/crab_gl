use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlProgram, Document, HtmlCanvasElement};
use wasm_bindgen::JsValue;
use crate::renderer::setup_shaders;
use crate::math::Vec4f;
use crate::renderer::circle_buffer::InstanceData;

use super::CircleBuffer;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = ANGLEInstancedArrays)]
    pub type AngleInstancedArrays;

    #[wasm_bindgen(method, getter, js_name = VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE)]
    pub fn vertex_attrib_array_divisor_angle(this: &AngleInstancedArrays) -> i32;

    #[wasm_bindgen(method, catch, js_name = drawArraysInstancedANGLE)]
    pub fn draw_arrays_instanced_angle(this: &AngleInstancedArrays, mode: u32, first: i32, count: i32, primcount: i32) -> Result<(), JsValue>;

    // TODO offset should be i64
    #[wasm_bindgen(method, catch, js_name = drawElementsInstancedANGLE)]
    pub fn draw_elements_instanced_angle(this: &AngleInstancedArrays, mode: u32, count: i32, type_: u32, offset: i32, primcount: i32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = vertexAttribDivisorANGLE)]
    pub fn vertex_attrib_divisor_angle(this: &AngleInstancedArrays, index: u32, divisor: u32);
}

#[allow(unused)]
pub struct Context{
    canvas_id: String,
    document: Document,
    canvas: HtmlCanvasElement,
    context: WebGlRenderingContext,
    active_shader: WebGlProgram,
    ext: AngleInstancedArrays,
    buffer: CircleBuffer,
    current_color: Vec4f
}

impl Context {
    pub fn new(canvas_id: &str) -> Result<Context, JsValue> {
        let canvas_id: String = canvas_id.to_string();
        let document = Self::get_document();
        let canvas = Self::get_canvas(&document, &canvas_id);
        let context = Self::get_rendering_context(&canvas);
        let active_shader = setup_shaders(&context).unwrap();
        let ext = Self::get_angle_extension(&context);
        let buffer = CircleBuffer::new(&context, &active_shader, &ext);
        let current_color = Vec4f::new(0.5, 0.7, 0.4, 1.0);
        Context::set_color(&context, &active_shader, &current_color);
        context.viewport(
            0,
            0,
            canvas.width().try_into().unwrap(),
            canvas.height().try_into().unwrap(),
        );

        return Ok(Context{
            canvas_id,
            document,
            canvas,
            context,
            active_shader,
            ext,
            buffer,
            current_color
        });
    }

    pub fn get_gl_context(&self) -> &WebGlRenderingContext{
        return &self.context;
    }

    pub fn get_active_shader(&self) -> &WebGlProgram{
        return &self.active_shader;
    }

    pub fn set_color(context: &WebGlRenderingContext, shader: &WebGlProgram, color: &Vec4f) {
        let color_location = context
            .get_uniform_location(&shader, "fragColor")
            .unwrap();
        context.uniform4fv_with_f32_array(Some(&color_location), &color.as_float_array());
    }

    fn get_document() -> Document {
        return web_sys::window().unwrap().document().unwrap();
    }

    fn get_canvas(document: &Document, canvas_id: &str) -> HtmlCanvasElement {
        return document.get_element_by_id(canvas_id).unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    }

    fn get_rendering_context(canvas: &HtmlCanvasElement) -> WebGlRenderingContext {
        let gl: WebGlRenderingContext = canvas
            .get_context("webgl").unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>().unwrap();
        return gl;
    }

    fn get_angle_extension(gl: &WebGlRenderingContext) -> AngleInstancedArrays {
        let extension: js_sys::Object = gl.get_extension("ANGLE_instanced_arrays").expect("Unable to find ANGLE_instanced_arrays").unwrap();
        let ext: AngleInstancedArrays = extension.unchecked_into::<AngleInstancedArrays>();
        return ext;
    }

    pub fn dispatch_draw(&self){
        self.buffer.buffer_instances(&self.context);
        self.ext.draw_arrays_instanced_angle(WebGlRenderingContext::TRIANGLE_FAN, 0, 4, self.buffer.instances.len() as i32).expect("Failed to draw circles");
    }

    pub fn get_mutable_instances(&mut self) -> &mut Vec<InstanceData>{
        return self.buffer.get_mutable_instances();
    }
}
