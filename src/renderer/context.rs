use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlProgram};
use wasm_bindgen::JsValue;
use crate::renderer::setup_shaders;
use crate::math::Vec4f;

pub struct Context{
    context: WebGlRenderingContext,
    active_shader: WebGlProgram,
}

impl Context {
    pub fn new(canvas_id: &str) -> Result<Context, JsValue> {
        let context_handle: WebGlRenderingContext = Context::init_webgl_context(canvas_id).unwrap();
        let shader: WebGlProgram = setup_shaders(&context_handle).unwrap();
        context_handle.use_program(Some(&shader));
        return Ok(Context{
            context: context_handle,
            active_shader: shader
        });
    }

    pub fn get_gl_context(&self) -> &WebGlRenderingContext{
        return &self.context;
    }

    pub fn get_active_shader(&self) -> &WebGlProgram{
        return &self.active_shader;
    }

    #[allow(dead_code)]
    pub fn set_color(&self, color: &Vec4f) {
        let color_location = self.context
            .get_uniform_location(&self.active_shader, "fragColor")
            .unwrap();
        self.context.uniform4fv_with_f32_array(Some(&color_location), &color.as_float_array());
    }

    fn init_webgl_context(canvas_id: &str) -> Result<WebGlRenderingContext, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        let gl: WebGlRenderingContext = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();

        gl.viewport(
            0,
            0,
            canvas.width().try_into().unwrap(),
            canvas.height().try_into().unwrap(),
        );
        return Ok(gl);
    }
}
