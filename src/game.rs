use glow::*;
use glutin::prelude::GlDisplay;


pub struct Game {
    renderer: Renderer,
    // game_state: GameState,
}

impl Game {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        let renderer = Renderer::new(gl_display);


        Game {
            renderer,
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        self.renderer.resize(width, height);
    }

    pub fn update(&self) {
        self.renderer.draw();
    }
}

pub struct Renderer {
    gl: Context,
    program: NativeProgram,
    vbo: NativeBuffer,
    vao: NativeVertexArray,
}

impl Renderer {
    fn new<D: GlDisplay>(gl_display: &D) -> Self {
        unsafe{
            let gl = Context::from_loader_function_cstr(
                |s| gl_display.get_proc_address(s)
            );

            let program = gl.create_program().expect("Failed to create gl program");

            let vertex_shader = init_shader(&gl, glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
            let frag_shader = init_shader(&gl, glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, frag_shader);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            gl.use_program(Some(program));
            // gl.delete_shader(vertex_shader);
            // gl.delete_shader(frag_shader);

            let (vbo, vao) = create_vertex_buffer(&gl);

            let pos_attrib = gl.get_attrib_location(program, "position")
                .expect("Failed to find position location");
            let col_attrib = gl.get_attrib_location(program, "color")
                .expect("Failed to find color location");
            println!("DEBUG: {:?}, {:?}", pos_attrib, col_attrib);

            gl.enable_vertex_attrib_array(pos_attrib);
            gl.vertex_attrib_pointer_f32(
                pos_attrib, 
                2, 
                FLOAT, 
                false, 
                std::mem::size_of::<[f32;5]>() as i32, 
                0
            );

            gl.enable_vertex_attrib_array(col_attrib);
            gl.vertex_attrib_pointer_f32(
                col_attrib, 
                3, 
                FLOAT, 
                false, 
                std::mem::size_of::<[f32;5]>() as i32, 
                std::mem::size_of::<[f32;2]>() as i32, 

            ); 


            Renderer {
                gl,
                program,
                vbo,
                vao
            }
        }
    }

    fn draw(&self) {
        self.draw_with_clear_color(0.0, 0.0, 0.0, 1.0);
    }

    fn draw_with_clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            self.gl.clear(COLOR_BUFFER_BIT);
            self.gl.clear_color(red, green, blue, alpha);
            self.gl.use_program(Some(self.program));

            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.draw_arrays(TRIANGLES, 0, 3);
        }
    }

    fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.viewport(0, 0, width, height);
        }
    }
}

unsafe fn init_shader(gl: &Context, shader_type: u32, shader_source: &str) -> Shader {
    let shader: Shader = gl.create_shader(shader_type)
        .expect(&format!("Failed to create shader: {}", shader_type));

    gl.shader_source(shader, shader_source);
    gl.compile_shader(shader);

    if !gl.get_shader_compile_status(shader) {
        panic!("{}", gl.get_shader_info_log(shader));
    } else {
        shader
    }
}

unsafe fn create_vertex_buffer(gl: &Context) -> (NativeBuffer, NativeVertexArray) {
    let bytes: &[u8] = core::slice::from_raw_parts(
        VERTEX_DATA.as_ptr() as *const u8,
        VERTEX_DATA.len() * core::mem::size_of::<f32>()
    );

    let vbo = gl.create_buffer()
        .expect("Failed to create vertex buffer object");

    gl.bind_buffer(ARRAY_BUFFER, Some(vbo));

    gl.buffer_data_u8_slice(ARRAY_BUFFER, bytes, STATIC_DRAW);

    let vao = gl.create_vertex_array()
        .expect("Failed to create vertex array object");

    gl.bind_vertex_array(Some(vao));

    (vbo, vao)
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
    0.0,  0.5,  0.0,  1.0,  0.0,
    0.5, -0.5,  0.0,  0.0,  1.0,
];

const VERTEX_SHADER_SOURCE: &str = "
#version 100
precision mediump float;

attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
\0";

const FRAGMENT_SHADER_SOURCE: &str = "
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";

