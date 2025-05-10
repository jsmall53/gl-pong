use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::ops::Deref;
use std::cell::RefCell;

use glow::*;
use glutin::prelude::GlDisplay;
use nalgebra_glm as glm;

pub struct Game {
    renderer: Renderer,
    game_state: GameState,
    frame_counter: FrameCounter,
}

impl Game {
    pub fn new<D: GlDisplay>(gl_display: &D, width: i32, height: i32) -> Self {
        let game_state = GameState::new();
        let renderer = Renderer::new(gl_display, width, height, &game_state);
        println!("{:?}", renderer);

        Game {
            renderer,
            game_state,
            frame_counter: FrameCounter::new(),
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.renderer.resize(width, height);
    }

    pub fn update(&mut self) {
        self.update_frames();

        self.game_state.update();
        self.renderer.draw(&self.game_state);
    }

    fn update_frames(&mut self) {
        self.frame_counter.increment();
        match self.frame_counter.fps() {
            Some(fps) => println!("{:.2} fps", fps),
            None => { }
        }
    }
}

/*
 * Ball is going to need velocity, acceleration,
 * position, direction(?)
 * */

struct GameState {
    paddles: Vec<Paddle>,
}

impl GameState {
    fn new() -> Self {
        let mut next_paddle_id = 0; // this is so stupid lol
        let x_pos = 0.5;
        let paddle_width = 0.03;
        let paddle_height = 0.2;
        let left_paddle = Paddle::new(next_paddle_id, glm::Vec2::new(-x_pos, 0.0), paddle_width, paddle_height);
        next_paddle_id += 1;
        let right_paddle = Paddle::new(next_paddle_id, glm::Vec2::new(x_pos - paddle_width, 0.0), paddle_width, paddle_height);
        next_paddle_id += 1;

        GameState {
            paddles: vec![left_paddle, right_paddle],
        }
    }

    fn paddles(&self) -> &[Paddle] {
        &self.paddles
    }

    fn update(&mut self) {
        // let y_movement = 45.0f32.sin();
        // for paddle in &mut self.paddles {
        //    paddle.move_y(y_movement);
        // }
    }
}

pub struct Paddle {
    id: u64,
    width: f32,
    height: f32,
    position: glm::Vec2,
    vertices: [f32;30],
}

impl Paddle {
    fn new(id: u64, position: glm::Vec2, width: f32, height: f32) -> Self {

        let x1 = position.x;
        let x2 = position.x + width;
        let y1 = position.y;
        let y2 = position.y + height;

        let vertices: [f32;30] = [
            x1, y1,  1.0,  1.0,  1.0,
            x2, y2,  1.0,  1.0,  1.0,
            x2, y1,  1.0,  1.0,  1.0,

            x1, y1,  1.0,  1.0,  1.0,
            x2, y2,  1.0,  1.0,  1.0,
            x1, y2,  1.0,  1.0,  1.0,
        ];

        let mut paddle = Paddle {
            id,
            width,
            height,
            position,
            vertices, 
        };
        Self::clamp_position(&mut paddle.position, &width, &height);
        
        return paddle
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn move_y(&mut self, distance: f32) {
        self.move_by_distance(glm::Vec2::new(0.0, distance));
    }

    fn move_by_distance(&mut self, distance: glm::Vec2) {
        self.position.x += distance.x;
        self.position.y += distance.y;
        Self::clamp_position(&mut self.position, &self.width, &self.height);
        // self.update_vertices();
    }

    fn update_vertices(&mut self) {
        let x1 = self.position.x;
        let x2 = self.position.x + self.width;
        let y1 = self.position.y;
        let y2 = self.position.y + self.height;

        self.vertices[0] = x1;
        self.vertices[1] = y1;
        self.vertices[5] = x2;
        self.vertices[6] = y2;
        self.vertices[10] = x2;
        self.vertices[11] = y1;

        self.vertices[15] = x1;
        self.vertices[16] = y1;
        self.vertices[20] = x2;
        self.vertices[21] = y2;
        self.vertices[25] = x1;
        self.vertices[26] = y2;
    }

    fn clamp_position(position: &mut glm::Vec2, width: &f32, height: &f32) {
        if position.x < -1.0 {
            position.x = -1.0;
        } else if position.x + width > 1.0 {
            position.x = 1.0
        }

        if position.y < -1.0 {
            position.y = -1.0;
        } else if position.y + height > 1.0 {
            position.y = 1.0;
        }
    }
}

pub struct Ball {
    radius: f32,
    position: glm::Vec2,
    velocity: glm::Vec2, // (speed, angle)
}

impl Ball {

}

#[derive(Debug)]
pub struct Renderer {
    gl: Context,

    width: i32,
    height: i32,

    paddle_program: NativeProgram,
    ball_program: NativeProgram,

    paddle_data: HashMap<u64, (NativeBuffer, NativeVertexArray)>,
    paddle_mvp: NativeUniformLocation,

    ball_vbo: NativeBuffer,
    ball_vao: NativeVertexArray,
    ball_radius: NativeUniformLocation,
}

impl Renderer {
    fn new<D: GlDisplay>(gl_display: &D, width: i32, height: i32, game_state: &GameState) -> Self {
        unsafe{
            let gl = Context::from_loader_function_cstr(
                |s| gl_display.get_proc_address(s)
            );
            gl.enable(PROGRAM_POINT_SIZE);
            gl.viewport(0, 0, width, height);

            let paddle_program = init_program(&gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);

            gl.use_program(Some(paddle_program));
            let pos_attrib = gl.get_attrib_location(paddle_program, "position")
                .expect("Failed to find position location");
            let col_attrib = gl.get_attrib_location(paddle_program, "color")
                .expect("Failed to find color location");
            let paddle_mvp = gl.get_uniform_location(paddle_program, "MVP")
                .expect("Failed to find paddle mvp location");

            let mut paddle_data = HashMap::new();
            for paddle in game_state.paddles() {
                let vertexes = create_paddle_buffer(
                    &gl, pos_attrib, col_attrib, paddle.vertices()
                );

                paddle_data.insert(paddle.id(), vertexes);
            }

            let ball_program = init_program(&gl, BALL_VSHADER_SOURCE, BALL_FSHADER_SOURCE);

            gl.use_program(Some(ball_program));
            let ball_pos = gl.get_attrib_location(ball_program, "ballPosition")
                .expect("Failed to find ball position uniform");
            let ball_col = gl.get_attrib_location(ball_program, "ballColor")
                .expect("Failed to find ball color uniform localtion");
            let ball_radius = gl.get_uniform_location(ball_program, "ballRadius")
                .expect("Failed to find ball radius uniform location");
            let (ball_vbo, ball_vao) = 
                create_paddle_buffer(&gl, ball_pos, ball_col, &BALL_VERTICES);

            gl.use_program(None);
            Renderer {
                gl,
                width,
                height,
                paddle_program,
                ball_program,
                paddle_data,
                paddle_mvp,
                ball_vbo,
                ball_vao,
                ball_radius,
            }
        }
    }

    fn draw(&self, game_state: &GameState) { // TODO: take in ball and paddle from game state so we can draw accurately...
        unsafe {
            self.gl.clear(COLOR_BUFFER_BIT);
            self.gl.clear_color(0.2, 0.5, 0.2, 1.0);

            self.gl.use_program(Some(self.ball_program));
            self.draw_ball();

            self.gl.use_program(Some(self.paddle_program));
            self.draw_paddles(game_state.paddles());

            self.gl.use_program(None);
            self.bind_vertex_array(None);
        }
    }
    
    unsafe fn draw_paddles(&self, paddles: &[Paddle]) {
        unsafe {
            for paddle in paddles {
                if let Some((_, vao)) = self.paddle_data.get(&paddle.id()) {
                    let position = &paddle.position;

                    let ratio: f32 = self.width as f32 / self.height as f32;
                    let mut m = glm::Mat4::identity();
                    let pos = &glm::Vec3::new(position.x * ratio, position.y, 0.0);
                    println!("{}, {:?}", ratio, pos);
                    let m2 = glm::translate(
                        &glm::Mat4::identity(),
                        pos,
                    );

                    let p = glm::ortho(-ratio, ratio, -1.0, 1.0, 1.0, -1.0);
                    // let p = glm::ortho(0.0, self.width as f32, 0.0, self.height as f32, 1.0, -1.0);
                    let mvp = p * m2;

                    
                    self.gl.uniform_matrix_4_f32_slice(Some(&self.paddle_mvp), false, mvp.as_slice());
                    self.gl.bind_vertex_array(Some(*vao));
                    self.gl.draw_arrays(TRIANGLES, 0, 6);
                }
            }
        }
    }

    unsafe fn draw_ball(&self) {
        self.gl.uniform_1_f32(Some(&self.ball_radius), 10.0);
        self.gl.bind_vertex_array(Some(self.ball_vao));

        self.gl.draw_arrays(POINTS, 0, 1);
    }

    fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            self.width = width;
            self.height = height;
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

            for  (_, (vbo, vao)) in self.paddle_data.iter() {
                self.gl.delete_buffer(*vbo);
                self.gl.delete_vertex_array(*vao);
            }
            self.paddle_data.clear();
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

// #[rustfmt::skip]
// static LEFT_PADDLE_VERTICES: [f32;30] = [
//     -0.99, -0.1,  1.0,  1.0,  1.0,
//     -0.96,  0.1,  1.0,  1.0,  1.0,
//     -0.96, -0.1,  1.0,  1.0,  1.0,
//
//     -0.99, -0.1,  1.0,  1.0,  1.0,
//     -0.96,  0.1,  1.0,  1.0,  1.0,
//     -0.99,  0.1,  1.0,  1.0,  1.0,
// ];

// #[rustfmt::skip]
// static RIGHT_PADDLE_VERTICES: [f32;30] = [
//     0.99, -0.1,  1.0,  1.0,  1.0,
//     0.96,  0.1,  1.0,  1.0,  1.0,
//     0.96, -0.1,  1.0,  1.0,  1.0,
//
//     0.99, -0.1,  1.0,  1.0,  1.0,
//     0.96,  0.1,  1.0,  1.0,  1.0,
//     0.99,  0.1,  1.0,  1.0,  1.0,
// ];

#[rustfmt::skip]
static BALL_VERTICES: [f32;5] = [
    0.0, 0.0, 1.0, 1.0, 1.0,
];

const VERTEX_SHADER_SOURCE: &str = "
#version 100
// precision mediump float;

uniform mat4 MVP;
attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = MVP * vec4(position, 0.0, 1.0);
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
attribute vec2 ballPosition;
attribute vec3 ballColor;
uniform float ballRadius;

varying vec3 v_color;

// uniform mat4 projection;

void main() {
    // gl_Position = projection * vec4(ballPosition, 0.0, 1.0);
    gl_Position = vec4(ballPosition, 0.0, 1.0);
    gl_PointSize = ballRadius * 2.0; // diameter
    v_color = ballColor;
}
\0";

const BALL_FSHADER_SOURCE: &str = "
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    vec2 coord = gl_PointCoord * 2.0 - 1.0;
    float dist = length(coord);
    float alpha = smoothstep(1.0, 0.98, 1.0 - dist); // Soft edge (anti-aliasing)

    if (dist > 1.0)
        discard;

    gl_FragColor = vec4(v_color, alpha);
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
