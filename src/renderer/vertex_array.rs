use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlBuffer};

struct VertexBuffer{
    handle: WebGlBuffer
}

impl VertexBuffer{
    fn new(gl: &WebGlRenderingContext, vertices: &[f32], shader_program: &WebGlProgram) -> VertexBuffer {
        // Create the OpenGL bufer and get a handle
        let id: WebGlBuffer = gl.create_buffer().unwrap();

        // Bind buffer
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&id));

        // Allocate initial memory + copy data
        let vertex_array = unsafe { js_sys::Float32Array::view(&vertices) };
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertex_array,
            WebGlRenderingContext::STATIC_DRAW,
        );

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&id)); //TODO: Remove this line?

        // Enable the coordinates attribute
        let coordinates_location = gl.get_attrib_location(&shader_program, "coordinates");
        gl.vertex_attrib_pointer_with_i32(
            coordinates_location as u32,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        gl.enable_vertex_attrib_array(coordinates_location as u32);
        return VertexBuffer { handle: id }
    }

    fn bind(&self, gl: &WebGlRenderingContext){
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.handle));
    }

    fn buffer_data(gl: &WebGlRenderingContext, vertices: &[f32]){
        let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
}

pub fn setup_vertices(gl: &WebGlRenderingContext, vertices: &[f32], shader_program: &WebGlProgram) {
    let vertex_buffer = VertexBuffer::new(gl, vertices, shader_program);
}