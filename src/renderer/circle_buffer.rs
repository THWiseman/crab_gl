use web_sys::{WebGlRenderingContext, WebGlProgram, WebGlBuffer};
use crate::game;
use crate::game::world::world_to_gl;
use crate::math::{Vec3f, Vec2f};
use crate::renderer::context::AngleInstancedArrays;
use crate::util::log;

pub struct VertexData{
    pub position: Vec3f
}

#[derive(Copy, Clone)]
pub struct InstanceData{
    pub center_x: f32,
    pub center_y: f32,
}

#[allow(unused)]
pub struct CircleBuffer{
    pub vertices: Vec<VertexData>,
    vertex_buffer: WebGlBuffer,
    pub instances: Vec<InstanceData>,
    instance_array: WebGlBuffer
}

impl CircleBuffer{
    pub fn new(gl: &WebGlRenderingContext, shader_program: &WebGlProgram, ext: &AngleInstancedArrays) -> CircleBuffer {
        //static vertices for the circle
        let a = world_to_gl(game::game_state::DEFAULT_BOUNDS, Vec2f::new(0., 0.));
        let b = world_to_gl(game::game_state::DEFAULT_BOUNDS, Vec2f::new(game::game_state::DEFAULT_PARTICLE_RADIUS, 0.));
        let radius = (b.subtract(&a)).length();
        log(&format!("Radiu: {:?}", radius), crate::util::LogLevel::Warning);
        let vertices = CircleBuffer::generate_circle_points(Vec3f::new(0., 0., 0.), radius, 32);
        let vertex_buffer = CircleBuffer::new_vertex_array(gl, &vertices, shader_program);

        //buffer for the instance data that will change every frame
        let instances: Vec<InstanceData> = vec![InstanceData{center_x: 0.0, center_y: 0.0}; game::game_state::DEFAULT_MAX_PARTICLES];
        let instance_array = CircleBuffer::new_instance_array(gl, CircleBuffer::get_buffer_ref(&instances), shader_program, ext);
        return CircleBuffer{vertices,
                            vertex_buffer,
                            instances,
                            instance_array,
                            };
    }

    // cast Vec<T> -> &[f32]
    fn get_buffer_ref<T>(arg: &Vec<T>) -> &[f32] {
        let ptr = arg.as_slice().as_ptr() as *const f32;
        let len = arg.len() * std::mem::size_of::<T>() / std::mem::size_of::<f32>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    fn generate_circle_points(center: Vec3f, radius: f32, n: usize) -> Vec<VertexData> {
        let mut points = Vec::new();
    
        for i in 0..n {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (n as f32);
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            let z = 0.;
            points.push(VertexData{position:Vec3f{ x, y, z }});
        }
    
        points
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

    fn new_instance_array(gl: &WebGlRenderingContext, vertices: &[f32], shader_program: &WebGlProgram, ext: &AngleInstancedArrays) -> WebGlBuffer {
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
        ext.vertex_attrib_divisor_angle(offset_attrib, 1);
        return id;
    }

    pub fn get_mutable_instances(&mut self) -> &mut Vec<InstanceData>{
        return &mut self.instances;
    }

    pub fn buffer_instances(&self, gl: &WebGlRenderingContext){
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.instance_array));
        let instances_array = unsafe { js_sys::Float32Array::view(CircleBuffer::get_buffer_ref(&self.instances)) };
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &instances_array,
            WebGlRenderingContext::DYNAMIC_DRAW,
        );
    }
}