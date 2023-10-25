use wasm_bindgen::JsCast;
use web_sys::{AngleInstancedArrays, WebGlRenderingContext, WebGlProgram, WebGlBuffer};
use crate::math::Vec3f;

pub struct VertexData{
    pub position: Vec3f
}

impl VertexData{
    pub fn as_float_array(&self) -> [f32; 3] {
        [self.position.x, self.position.y, self.position.z]
    }
}
#[derive(Copy, Clone)]
pub struct InstanceData{
    pub center_x: f32,
    pub center_y: f32,
}

pub struct CircleBuffer{
    pub vertices: Vec<VertexData>,
    vertex_buffer: WebGlBuffer,
    pub instances: Vec<InstanceData>,
    instance_array: WebGlBuffer, 
    pub ext: AngleInstancedArrays,
}

impl CircleBuffer{
    const MAX_CIRCLES: usize = 1000;

    pub fn new(gl: &WebGlRenderingContext, shader_program: &WebGlProgram) -> CircleBuffer {
        // enable the instanced rendering extension
        let ext: AngleInstancedArrays = gl.get_extension("ANGLE_instanced_arrays").unwrap().unwrap().dyn_into().unwrap();

        //static vertices for the circle
        let vertices = CircleBuffer::new_vertex_data();
        let vertex_buffer = CircleBuffer::new_vertex_array(gl, &vertices, shader_program);

        //buffer for the instance data that will change every frame
        let instances = vec![InstanceData{center_x: 0.0, center_y: 0.0}; CircleBuffer::MAX_CIRCLES];
        let instance_array = CircleBuffer::new_instance_array(gl, CircleBuffer::get_buffer_ref(&instances), shader_program);
        return CircleBuffer{vertices,
                            vertex_buffer,
                            instances,
                            instance_array,
                            ext};
    }

    // cast Vec<T> -> &[f32]
    fn get_buffer_ref<T>(arg: &Vec<T>) -> &[f32] {
        let ptr = arg.as_slice().as_ptr() as *const f32;
        let len = arg.len() * std::mem::size_of::<T>() / std::mem::size_of::<f32>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    fn new_vertex_data() -> Vec<VertexData> {
        let v0 = VertexData{position: Vec3f::new(-0.1, -0.1, 0.0)};
        let v1 = VertexData{position: Vec3f::new(0.1, -0.1, 0.0)};
        let v2 = VertexData{position: Vec3f::new(0.1, 0.1, 0.0)};
        let v3 = VertexData{position: Vec3f::new(-0.1, 0.1, 0.0)};
        return vec![v0, v1, v2, v3];
    }

    fn new_vertex_array(gl: &WebGlRenderingContext, vertices: &Vec<VertexData>, shader_program: &WebGlProgram) -> WebGlBuffer {
        // Create the OpenGL bufer and get a handle
        let id: WebGlBuffer = gl.create_buffer().unwrap();

        // Bind buffer
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&id));

        // Allocate initial memory + copy data
        let vertex_array = unsafe { js_sys::Float32Array::view(CircleBuffer::get_buffer_ref(vertices)) };
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertex_array,
            WebGlRenderingContext::STATIC_DRAW,
        );

        // Enable the position attribute
        let position_attrib = gl.get_attrib_location(&shader_program, "position");
        gl.vertex_attrib_pointer_with_i32(
            position_attrib as u32,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        gl.enable_vertex_attrib_array(position_attrib as u32);

        return id;
    }

    fn new_instance_array(gl: &WebGlRenderingContext, vertices: &[f32], shader_program: &WebGlProgram) -> WebGlBuffer {
        // Create the OpenGL bufer and get a handle
        let id: WebGlBuffer = gl.create_buffer().unwrap();

        // Bind buffer
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&id));

        // Allocate initial memory + copy data
        let instance_array = unsafe { js_sys::Float32Array::view(&vertices) };
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &instance_array,
            WebGlRenderingContext::DYNAMIC_DRAW,
        );

        let offset_attrib = gl.get_attrib_location(&shader_program, "offset") as u32;
        gl.enable_vertex_attrib_array(offset_attrib);
        gl.vertex_attrib_pointer_with_i32(offset_attrib, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        let etx_angle: web_sys::AngleInstancedArrays = WebGlRenderingContext::get_extension(&gl, "ANGLE_instanced_arrays").unwrap().unwrap().dyn_into().unwrap();
        etx_angle.vertex_attrib_divisor_angle(offset_attrib, 1);
        return id;
    }

    #[allow(dead_code)]
    fn bind(&self, gl: &WebGlRenderingContext){
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.vertex_buffer));
    }

    #[allow(dead_code)]
    fn buffer_data(gl: &WebGlRenderingContext, vertices: &[f32]){
        let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    pub fn render(&self, gl: &WebGlRenderingContext){
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.instance_array));
        let instances_array = unsafe { js_sys::Float32Array::view(CircleBuffer::get_buffer_ref(&self.instances)) };
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &instances_array,
            WebGlRenderingContext::DYNAMIC_DRAW,
        );
        self.ext.draw_arrays_instanced_angle(WebGlRenderingContext::TRIANGLE_FAN, 0, 4, self.instances.len() as i32);
    }
}