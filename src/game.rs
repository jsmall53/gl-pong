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
        let delta = self.update_frames();

        self.game_state.update(delta);
        self.renderer.draw(&self.game_state);
    }

    fn update_frames(&mut self) -> f32 {
        let delta = self.frame_counter.increment();
        match self.frame_counter.fps() {
            Some(fps) => println!("{:.2} fps", fps),
            None => { }
        }
        delta
    }
}

/*
 * Ball is going to need velocity, acceleration,
 * position, direction(?)
 * */

struct GameState {
    start_time: Instant,
    paddles: Vec<Paddle>,
    balls: Vec<Ball>,
}

impl GameState {
    fn new() -> Self {
        let mut next_item_id = 0; // this is so stupid lol
        let x_pos = 0.99;
        let left_paddle = Paddle::new(next_item_id, glm::Vec2::new(-x_pos + (PADDLE_WIDTH / 2.0f32), 0.0), PADDLE_WIDTH, PADDLE_HEIGHT);
        next_item_id += 1;
        let right_paddle = Paddle::new(next_item_id, glm::Vec2::new(x_pos - (PADDLE_WIDTH / 2.0f32), 0.0), PADDLE_WIDTH, PADDLE_HEIGHT);
        next_item_id += 1;

        let ball = Ball::new(next_item_id, 10.0);
        GameState {
            start_time: Instant::now(),
            paddles: vec![left_paddle, right_paddle],
            balls: vec![ball],
        }
    }

    fn paddles(&self) -> &[Paddle] {
        &self.paddles
    }
    
    fn balls(&self) -> &[Ball] {
        &self.balls
    }

    fn update(&mut self, delta: f32) {
        let time = self.start_time.elapsed().as_secs_f32();

        for ball in &mut self.balls {
            ball.apply_velocity(delta);
            // TODO: resolve collisions
            // TODO: clamp speed
        }

        let y_movement = (time * std::f32::consts::PI * 0.3).sin();
        println!("{}", y_movement);
        for paddle in &mut self.paddles {
           paddle.move_y(y_movement);
        }
    }

    fn resolve_collisions(&mut self, ball: &Ball, ) {
        // ceiling surface should be Vec2(1.0, -1.0) for surface normal pointing down.
        // floor surface should be Vec2(-1.0, 1.0) for surface normal pointing up
    }
}

pub struct Paddle {
    id: u64,
    width: f32,
    height: f32,
    position: glm::Vec2,
    vertices: [f32;30],
}

static X1_PADDLE: f32 = -0.015;
static X2_PADDLE: f32 = 0.015;
static Y1_PADDLE: f32 = -0.1;
static Y2_PADDLE: f32 = 0.1;

static PADDLE_WIDTH: f32 = X2_PADDLE - X1_PADDLE;
static PADDLE_HEIGHT: f32 = Y2_PADDLE - Y1_PADDLE;

static PADDLE_VERTICES: [f32;30] = [
    X1_PADDLE, Y1_PADDLE,  1.0,  1.0,  1.0,
    X2_PADDLE, Y2_PADDLE,  1.0,  1.0,  1.0,
    X2_PADDLE, Y1_PADDLE,  1.0,  1.0,  1.0,

    X1_PADDLE, Y1_PADDLE,  1.0,  1.0,  1.0,
    X2_PADDLE, Y2_PADDLE,  1.0,  1.0,  1.0,
    X1_PADDLE, Y2_PADDLE,  1.0,  1.0,  1.0,
];

impl Paddle {
    fn new(id: u64, position: glm::Vec2, width: f32, height: f32) -> Self {

        let mut paddle = Paddle {
            id,
            width,
            height,
            position,
            vertices: PADDLE_VERTICES.clone(), 
        };
        paddle.clamp_position();
        
        return paddle
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn move_y(&mut self, y_pos: f32) {
        self.move_position(glm::Vec2::new(self.position.x, y_pos));
    }

    fn move_position(&mut self, new_pos: glm::Vec2) {
        self.position = new_pos;
        self.clamp_position();
    }

    fn clamp_position(&mut self) {
        self.position = clamp_position_2d(
            self.position,
            -1.0,
            1.0,
            -1.0,
            1.0,
            self.width / 2.0,
            self.height / 2.0,
        );
    }

    // TODO: fix this so that surface is relative to another object instead of hardcoded left and
    // right. Currently the left paddle will always use the right edge for the surface and right
    // paddle will always use left edge for the surface. A surface should be relative to another
    // object (ball in our case). If the object is the right of this paddle use right edge, if the object
    // is to the left of this paddle use left edge, above us. For regular pong only vertical
    // surfaces are relevant. Maybe a better version would be to construct a list of surfaces from
    // each object and just do collision resolution for each surface. Might have funny cases for
    // corner hits and such that would make this interesting...
    // New parameter should be a position vector that we can compare to our current pos.
    // let dir = (incoming.x - self.pos.x).signum()
    fn surface(&self) -> Surface {
        let direction = self.position.x.signum();
        let y_offset = self.height / 2.0f32;
        let x_offset = self.width / 2.0f32;

        let mut a = glm::Vec2::new(0.0, 0.0);
        let mut b = glm::Vec2::new(0.0, 0.0);

        a.y = self.position.y - y_offset;
        b.y = self.position.y + y_offset;

        let x = self.position.x + (direction * -x_offset);
        a.x = x;
        b.x = x;

        Surface { a, b }
    }
}

struct Surface {
    a: glm::Vec2,
    b: glm::Vec2,
}

pub struct Ball {
    id: u64,
    radius: f32,
    position: glm::Vec2,
    velocity: glm::Vec2, // (speed, angle)
    vertices: [f32;5],
}

static BALL_VERTICES: [f32;5] = [
    0.0, 0.0, 1.0, 1.0, 1.0,
];

impl Ball {
    fn new(id: u64, radius: f32) -> Self {
        Ball {
            id,
            radius,
            position: glm::Vec2::new(0.0, 0.0),
            velocity: glm::Vec2::new(0.5, 0.5),
            vertices: BALL_VERTICES.clone(),
        }
    }

    fn id(&self) -> u64 {
        self.id
    }
    
    fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn clamp_position(&mut self) {
        self.position = clamp_position_2d(
            self.position, 
            -1.0,
            1.0,
            -1.0,
            1.0,
            self.radius,
            self.radius
        )
    }

    fn apply_velocity(&mut self, delta: f32) {
        self.position += self.velocity * delta;
    }
}

fn clamp_position_2d(pos: glm::Vec2, x_min: f32, x_max: f32, y_min: f32, y_max: f32, x_offset: f32, y_offset: f32) -> glm::Vec2 {
    let mut out_pos = glm::Vec2::new(pos.x, pos.y);
    if pos.y - y_offset < y_min {
        out_pos.y = y_min + y_offset;
    } else if pos.y + y_offset > y_max {
        out_pos.y = y_max - y_offset;
    }

    if pos.x - x_offset < x_min {
        out_pos.x = x_min + x_offset;
    } else if pos.x + x_offset > x_max {
        out_pos.x = x_max - x_offset;
    }

    out_pos
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

    ball_data: HashMap<u64, (NativeBuffer, NativeVertexArray)>,
    ball_mvp: NativeUniformLocation,
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
            let paddle_mvp = gl.get_uniform_location(paddle_program, "u_MVP")
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
            let ball_radius = gl.get_uniform_location(ball_program, "u_ballRadius")
                .expect("Failed to find ball radius uniform location");
            let ball_mvp = gl.get_uniform_location(ball_program, "u_ballMVP")
                .expect("Failed to find ball MVP uniform location");

            let mut ball_data = HashMap::new();
            for ball in game_state.balls() {
                let vertexes = create_paddle_buffer(
                    &gl, ball_pos, ball_col, ball.vertices()
                );

                ball_data.insert(ball.id(), vertexes);
            }
            gl.use_program(None);
            Renderer {
                gl,
                width,
                height,
                paddle_program,
                ball_program,
                paddle_data,
                paddle_mvp,
                ball_data,
                ball_mvp,
                ball_radius,
            }
        }
    }

    fn draw(&self, game_state: &GameState) { // TODO: take in ball and paddle from game state so we can draw accurately...
        unsafe {
            self.gl.clear(COLOR_BUFFER_BIT);
            self.gl.clear_color(0.2, 0.5, 0.2, 1.0);

            self.gl.use_program(Some(self.ball_program));
            self.draw_balls(game_state.balls());

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
                    let position = &paddle.position; // TODO: safe position access

                    let ratio: f32 = self.width as f32 / self.height as f32;
                    let pos = &glm::Vec3::new(position.x * ratio, position.y, 0.0);

                    let m = glm::translate(
                        &glm::Mat4::identity(),
                        pos,
                    );

                    let p = self.ortho_2d(ratio);
                    let mvp = p * m;
                    
                    self.gl.uniform_matrix_4_f32_slice(Some(&self.paddle_mvp), false, mvp.as_slice());
                    self.gl.bind_vertex_array(Some(*vao));
                    self.gl.draw_arrays(TRIANGLES, 0, 6);
                }
            }
        }
    }

    unsafe fn draw_balls(&self, balls: &[Ball]) {
        unsafe {
            for ball in balls {
                if let Some((_, vao)) = self.ball_data.get(&ball.id()) {
                    let position = &ball.position; // TODO: safe position access
                    let ratio: f32 = self.width as f32 / self.height as f32; // TODO: only do this
                                                                             // once per loop, pass
                                                                             // as param.
                    let pos = &glm::Vec3::new(position.x * ratio, position.y, 0.0);

                    let m = glm::translate(
                        &glm::Mat4::identity(),
                        pos,
                    );

                    let p = self.ortho_2d(ratio);
                    let mvp = p * m;

                    self.gl.uniform_matrix_4_f32_slice(Some(&self.ball_mvp), false, mvp.as_slice());
                    self.gl.uniform_1_f32(Some(&self.ball_radius), ball.radius);
                    self.gl.bind_vertex_array(Some(*vao));
                    self.gl.draw_arrays(POINTS, 0, 1);
                }
            }
        }
    }

    fn ortho_2d(&self, aspect_ratio: f32) -> glm::Mat4 {
        glm::ortho(
            -aspect_ratio,
            aspect_ratio,
            -1.0,
            1.0,
            1.0,
            -1.0,
            )
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

const VERTEX_SHADER_SOURCE: &str = "
#version 100
// precision mediump float;

uniform mat4 u_MVP;
attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = u_MVP * vec4(position, 0.0, 1.0);
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

uniform mat4 u_ballMVP;
uniform float u_ballRadius;

attribute vec2 ballPosition;
attribute vec3 ballColor;

varying vec3 v_color;

void main() {
    gl_Position = u_ballMVP * vec4(ballPosition, 0.0, 1.0);
    gl_PointSize = u_ballRadius * 2.0; // diameter
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
    last_frame_time: Instant,
    last_update_time: Instant,
    last_update_val: f64,
}

impl FrameCounter {
    pub fn new() -> Self {
        FrameCounter {
            begin: Instant::now(),
            frame_count: 0,
            last_frame_time: Instant::now(),
            last_update_time: Instant::now(),
            last_update_val: 0.0f64,
        }
    }

    pub fn increment(&mut self) -> f32 {
        let delta = self.last_frame_time.elapsed().as_secs_f32();
        self.frame_count += 1;
        self.last_frame_time = Instant::now();
        delta
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
