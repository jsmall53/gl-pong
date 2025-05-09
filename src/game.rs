use std::time::{Instant, Duration};
use std::ops::Deref;

use glow::*;
use glutin::prelude::GlDisplay;
use nalgebra_glm as glm;


pub struct Game {
    renderer: Renderer,
    game_state: GameState,
    frame_counter: FrameCounter,
}

impl Game {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        let renderer = Renderer::new(gl_display);
        println!("{:?}", renderer);

        Game {
            renderer,
            game_state: GameState::new(),
            frame_counter: FrameCounter::new(),
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        self.renderer.resize(width, height);
    }

    pub fn update(&mut self) {
        self.update_frames();

        self.game_state.update();
        self.renderer.draw();
    }

    fn update_frames(&mut self) {
        self.frame_counter.increment();
        match self.frame_counter.fps() {
            Some(fps) => println!("{:.2} fps", fps),
            None => { }
        }
    }
}

#[derive(Debug)]
pub struct Renderer {
    gl: Context,

    paddle_program: NativeProgram,
    ball_program: NativeProgram,

    left_paddle_vbo: NativeBuffer,
    left_paddle_vao: NativeVertexArray,

    right_paddle_vbo: NativeBuffer,
    right_paddle_vao: NativeVertexArray,
}

impl Renderer {
    fn new<D: GlDisplay>(gl_display: &D) -> Self {
        unsafe{
            let gl = Context::from_loader_function_cstr(
                |s| gl_display.get_proc_address(s)
            );
            gl.enable(PROGRAM_POINT_SIZE);

            let paddle_program = init_program(&gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);

            gl.use_program(Some(paddle_program));
            let pos_attrib = gl.get_attrib_location(paddle_program, "position")
                .expect("Failed to find position location");
            let col_attrib = gl.get_attrib_location(paddle_program, "color")
                .expect("Failed to find color location");

            let (left_paddle_vbo, left_paddle_vao) = 
                create_paddle_buffer(&gl, pos_attrib, col_attrib, &LEFT_PADDLE_VERTICES);
            let (right_paddle_vbo, right_paddle_vao) =
                create_paddle_buffer(&gl, pos_attrib, col_attrib, &RIGHT_PADDLE_VERTICES);

            let ball_program = init_program(&gl, BALL_VSHADER_SOURCE, BALL_FSHADER_SOURCE);
            // TODO: setup the ball buffers

            Renderer {
                gl,
                paddle_program,
                ball_program,
                left_paddle_vbo,
                left_paddle_vao,
                right_paddle_vbo,
                right_paddle_vao,
            }
        }
    }

    fn draw(&self) {
        unsafe {
            self.gl.clear(COLOR_BUFFER_BIT);
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);

            self.gl.use_program(Some(self.paddle_program));
            self.draw_right_paddle();
            self.draw_left_paddle();

            self.gl.use_program(Some(self.ball_program));
            self.draw_ball();


            self.gl.use_program(None);
            self.bind_vertex_array(None);
        }
    }
    
    unsafe fn draw_left_paddle(&self) {
        self.gl.bind_vertex_array(Some(self.left_paddle_vao));
        // self.gl.bind_buffer(ARRAY_BUFFER, Some(self.left_paddle_vbo));
        self.gl.draw_arrays(TRIANGLES, 0, 6);
    }

    unsafe fn draw_right_paddle(&self) {
        self.gl.bind_vertex_array(Some(self.right_paddle_vao));
        // self.gl.bind_buffer(ARRAY_BUFFER, Some(self.right_paddle_vbo));
        self.gl.draw_arrays(TRIANGLES, 0, 6);
    }

    unsafe fn draw_ball(&self) {
        // TODO: implement this.
    }

    fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.viewport(0, 0, width, height);
        }
    }
}

impl Deref for Renderer {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.paddle_program);
            self.gl.delete_program(self.ball_program);
            self.gl.delete_buffer(self.left_paddle_vbo);
            self.gl.delete_buffer(self.right_paddle_vbo);
            self.gl.delete_vertex_array(self.left_paddle_vao);
            self.gl.delete_vertex_array(self.right_paddle_vao);
        }
    }
}

unsafe fn init_program(gl: &Context, vshader_src: &str, fshader_src: &str) -> NativeProgram {
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

unsafe fn create_paddle_buffer(gl: &Context, pos_loc: u32, col_loc: u32, vertices: &[f32]) -> (NativeBuffer, NativeVertexArray) {
    let bytes: &[u8] = core::slice::from_raw_parts(
        vertices.as_ptr() as *const u8,
        vertices.len() * core::mem::size_of::<f32>()
    );

    let vbo = gl.create_buffer()
        .expect("Failed to create vertex buffer object");

    gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(ARRAY_BUFFER, bytes, STATIC_DRAW);

    let vao = gl.create_vertex_array()
        .expect("Failed to create vertex array object");

    gl.bind_vertex_array(Some(vao));

    gl.enable_vertex_attrib_array(pos_loc);
    gl.vertex_attrib_pointer_f32(
        pos_loc, 
        2, 
        FLOAT, 
        false, 
        std::mem::size_of::<[f32;5]>() as i32, 
        0
    );

    gl.enable_vertex_attrib_array(col_loc);
    gl.vertex_attrib_pointer_f32(
        col_loc, 
        3, 
        FLOAT, 
        false, 
        std::mem::size_of::<[f32;5]>() as i32, 
        std::mem::size_of::<[f32;2]>() as i32, 

    ); 

    gl.bind_vertex_array(None);
    gl.bind_buffer(ARRAY_BUFFER, None);
    (vbo, vao)
}

#[rustfmt::skip]
static LEFT_PADDLE_VERTICES: [f32;30] = [
    -0.99, -0.1,  1.0,  1.0,  1.0,
    -0.96,  0.1,  1.0,  1.0,  1.0,
    -0.96, -0.1,  1.0,  1.0,  1.0,

    -0.99, -0.1,  1.0,  1.0,  1.0,
    -0.96,  0.1,  1.0,  1.0,  1.0,
    -0.99,  0.1,  1.0,  1.0,  1.0,
];

#[rustfmt::skip]
static RIGHT_PADDLE_VERTICES: [f32;30] = [
    0.99, -0.1,  1.0,  1.0,  1.0,
    0.96,  0.1,  1.0,  1.0,  1.0,
    0.96, -0.1,  1.0,  1.0,  1.0,

    0.99, -0.1,  1.0,  1.0,  1.0,
    0.96,  0.1,  1.0,  1.0,  1.0,
    0.99,  0.1,  1.0,  1.0,  1.0,
];

#[rustfmt::skip]
static BALL_VERTICES: [f32;5] = [
    0.5, 0.5, 1.0, 1.0, 1.0,
];

const VERTEX_SHADER_SOURCE: &str = "
#version 100
// precision mediump float;

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

const BALL_VSHADER_SOURCE: &str = "
#version 100

precision mediump float;
uniform vec2 ballPosition;
uniform float ballRadius;

uniform mat4 projection;

void main() {
    gl_Position = projection * vec4(ballPosition, 0.0, 1.0);
    gl_PointSize = ballRadius * 2.0; // diameter
}
\0";

const BALL_FSHADER_SOURCE: &str = "
#version 100
precision mediump float;

uniform vec3 ballColor;

void main() {
    vec2 coord = gl_PointCoord * 2.0 - 1.0;
    float dist = length(coord);
    float alpha = smoothstep(1.0, 0.98, 1.0 - dist); // Soft edge (anti-aliasing)

    if (dist > 1.0)
        discard;

    gl_FragColor = vec4(ballColor, alpha);
}
\0";

pub struct FrameCounter {
    begin: Instant, // when was this started.
    frame_count: u64,
    last_update_time: Instant,
    last_update_val: f64,
}

impl FrameCounter {
    pub fn new() -> Self {
        FrameCounter {
            begin: Instant::now(),
            frame_count: 0,
            last_update_time: Instant::now(),
            last_update_val: 0.0f64,
        }
    }

    pub fn increment(&mut self) {
        self.frame_count += 1;
    }

    pub fn fps(&mut self) -> Option<f64> {
        if self.last_update_time.elapsed().as_secs() > 2 {
            self.last_update_val = self.frame_count as f64/ self.begin.elapsed().as_secs() as f64;
            self.last_update_time = Instant::now();
            return Some(self.last_update_val);
        }
        None
    }
}

struct GameState {

}

impl GameState {
    fn new() -> Self {
        GameState {

        }
    }

    fn update(&mut self) {

    }
}


const PONG_SHADER_SOURCE: &str = "
#version 300 es
 
precision highp float;
precision highp sample2D;

in vec2 uv;
out vec4 out_color;

uniform vec2 u_resolution;
uniform float u_time;
uniform vec4 u_mouse;

uniform vec2 u_rectLeftPos;
uniform vec3 u_rectLeftColor;
uniform vec2 u_rectRightPos;
uniform vec3 u_rectRightColor;
uniform vec2 u_ballPos;
uniform float u_ballSize;

void main() {
    vec2 st = uv * vec2(u_resolution.x / u_resolution.y, 1.);

    float dist = distance(st, ballPos);
    if (dist <= ballSize) { // ball pixel
        out_color = vec4(1.0, 1.0, 1.0, 1.0) // white ball, could parameterize this
    } else if ((st.x >= rectPos.x && // paddle pixel
                st.x <= rectPos.x + rectSize.x &&
                st.y >= rectPos.y && st.y <= rectPos.y + rectSize.y) ||
               (st.x >= rightRectPos.x && 
                st.x <= rightRectPos.x + rightRectSIze.x &&
                st.y >= rightRectPos.y && st.y <= rightRectPos.y + rightRectSIze.y)) {
        out_color = vec4(1.0, 1.0, 1.0, 1.0); // white
    } else { // background pixel
        out_color = vec4(0.0, 0.0, 0.0, 0.0);
    }
}
";
