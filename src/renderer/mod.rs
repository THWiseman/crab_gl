pub mod shader;
pub use shader::{create_shader, setup_shaders};

pub mod context;
pub use context::init_webgl_context;

pub mod shader_source;

pub mod vertex_array;
pub use vertex_array::setup_vertices;