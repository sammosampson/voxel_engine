use glium::*;

pub fn get_vertex_shader_src() -> String {
    String::from_utf8_lossy(include_bytes!("../../../../shaders/vertex.glsl")).to_string()
}

pub fn get_fragment_shader_src() -> String {
    String::from_utf8_lossy(include_bytes!("../../../../shaders/fragment.glsl")).to_string()
}

pub fn create_shader_program(display: &Display) -> Result<Program, ProgramCreationError> {
    Program::from_source(display, &get_vertex_shader_src(), &get_fragment_shader_src(), None)
}