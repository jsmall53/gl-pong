use glow::{ HasContext };
use std::collections::HashMap;
use std::rc::Rc;


pub struct GLShader {
    gl: Rc<glow::Context>,
    name: String,
    sources: HashMap<u32, String>,
    program: glow::NativeProgram,
}



impl GLShader {
    pub fn new(gl: Rc<glow::Context>, name: &str, vertex_src: &str, fragment_src: &str) -> Self {
        let mut sources = HashMap::new();
        sources.insert(glow::VERTEX_SHADER, vertex_src.into());
        sources.insert(glow::FRAGMENT_SHADER, fragment_src.into());

        unsafe {
            let program = init_program(&gl, vertex_src, fragment_src);
            Self {
                gl,
                name: name.into(),
                sources,
                program,
            }

        }
    }
}



unsafe fn init_program(gl: &glow::Context, vshader_src: &str, fshader_src: &str) -> glow::NativeProgram {
    let program = gl.create_program().expect("Failed to create gl program");

    let vertex_shader = init_shader(&gl, glow::VERTEX_SHADER, vshader_src);
    let frag_shader = init_shader(&gl, glow::FRAGMENT_SHADER, fshader_src);

    gl.attach_shader(program, vertex_shader);
    gl.attach_shader(program, frag_shader);
    gl.link_program(program);
    if !gl.get_program_link_status(program) {
        panic!("{}", gl.get_program_info_log(program));
    }
    gl.delete_shader(vertex_shader);
    gl.delete_shader(frag_shader);

    program
}



unsafe fn init_shader(gl: &glow::Context, shader_type: u32, shader_source: &str) -> glow::Shader {
    let shader: glow::Shader = gl.create_shader(shader_type)
        .expect(&format!("Failed to create shader: {}", shader_type));

    gl.shader_source(shader, shader_source);
    gl.compile_shader(shader);

    if !gl.get_shader_compile_status(shader) {
        panic!("{}\n{}", gl.get_shader_info_log(shader), shader_source);
    } else {
        shader
    }
}
