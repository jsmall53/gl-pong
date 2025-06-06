mod buffer;
use buffer::*;

mod draw;
use draw::*;

mod shader;
use shader::*;

pub mod camera;
use camera::*;

pub mod texture;
use texture::*;

use nalgebra_glm as glm;
use glow;

use std::error::Error;
use std::rc::Rc;
use std::fmt;


pub enum RendererBackend {
    OpenGL(OpenGLRendererAPI),
    None,
}



#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct QuadVertex {
    position: glm::Vec3,
    color: glm::Vec4,
    tex_coord: glm::Vec2,
    tex_index: f32,
    tiling_factor: f32,
    entity_id: i32,
}



impl QuadVertex {
    fn new() -> Self {
        Self {
            position: glm::Vec3::zeros(),
            color: glm::Vec4::zeros(),
            tex_coord: glm::Vec2::zeros(),
            tex_index: 0.0,
            tiling_factor: 1.0,
            entity_id: -1,
        }
    }
}


#[derive(Debug)]
#[repr(C)]
struct CameraData {
    view_projection: glm::Mat4,
}

impl CameraData {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const CameraData as *const u8,
                std::mem::size_of::<CameraData>(),
            )
        }
    }
}


const QUAD_VERTEX_COUNT: usize = 4;
const MAX_QUADS: usize = 20;
const MAX_VERTICES: usize = MAX_QUADS * QUAD_VERTEX_COUNT;
const MAX_INDICES: usize = MAX_QUADS * 6;
const MAX_TEXTURE_SLOTS: usize = 32;

const TEXTURE_COORDS: [glm::Vec2; 4] = [
    glm::Vec2::new(0.0, 0.0),
    glm::Vec2::new(1.0, 0.0),
    glm::Vec2::new(1.0, 1.0),
    glm::Vec2::new(0.0, 1.0),
];

struct Renderer2DData {
    gl: Rc<glow::Context>,
    
    quad_vertex_array: Box<GLVertexArray>,
    quad_vertex_buffer: Box<GLVertexBuffer>,
    // quad_vertex_array: Box<dyn VertexArray<Item = GLVertexBuffer, Item2 = GLIndexBuffer>>,
    // quad_shader: i32, // TODO: shader implementations.

    // circle_vertex_array: Box<dyn VertexArray<Item = GLVertexBuffer>>,
    // circle_vertex_buffer: Box<dyn VertexBuffer>,
    // // circle_shader: i32, // TODO: SHADER
    //
    // line_vertex_array: Box<dyn VertexArray<Item = GLVertexBuffer>>,
    // line_vertex_buffer: Box<dyn VertexBuffer>,
    // // line_shader: i32,
    //
    // text_vertex_array: Box<dyn VertexArray<Item = GLVertexBuffer>>,
    // text_vertex_buffer: Box<dyn VertexBuffer>,
    // // text_shader: i32,

    quad_index_count: u32,
    quad_vertex_buffer_base: Box<[QuadVertex; MAX_VERTICES]>,
    quad_vertex_buffer_idx: usize, // index position of current quad_vertex_buffer_base
    quad_vertex_positions: [glm::Vec4; 4], 
    quad_shader: GLShader,

    texture_slots: Vec<GLTexture>,
    texture_slot_idx: usize, // TODO: 0 = white_texture???

    camera_uniform_buffer: Box<GLUniformBuffer>,
    camera_data: CameraData,
}


#[derive(Debug)]
struct RenderStats {
    draw_calls: usize,
    quad_count: usize,
}



pub struct Renderer2D {
    data: Box<Renderer2DData>,
    backend: RendererBackend,
    stats: RenderStats,
}



impl Renderer2D {
    pub fn new(gl: glow::Context, width: i32, height: i32) -> Self {

        let quad_layout = BufferLayoutBuilder::new()
            .element(BufferElement::new(ShaderDataType::Float3, "a_Position", false))
            .element(BufferElement::new(ShaderDataType::Float4, "a_Color", false))
            .element(BufferElement::new(ShaderDataType::Float2, "a_TexCoord", false))
            .element(BufferElement::new(ShaderDataType::Float, "a_TexIndex", false))
            .element(BufferElement::new(ShaderDataType::Float, "a_TilingFactor", false))
            .element(BufferElement::new(ShaderDataType::Int, "a_EntityId", false))
            .build();
        println!("{:?}", quad_layout);
        let gl_rc = Rc::new(gl);
        let mut quad_buffer = GLVertexBuffer::new(
            gl_rc.clone(), 
            quad_layout,
            (std::mem::size_of::<QuadVertex>() * MAX_VERTICES) as i32,
        );
        let mut quad_vertex_array = GLVertexArray::new(gl_rc.clone());
        quad_vertex_array.add_vertex_buffer(&mut quad_buffer);


        // I didn't come up with this index array,
        // however, when using an index buffer the repeated
        // vertices aren't stored separately, so this is
        // essentially telling the GPU where to find the vertex
        // data in the vertex buffer. See 'quad_vertices' below
        // to see the vertex for each corresponding index
        // there are 4 different vertices
        let mut quad_indices: [u32; MAX_INDICES] = [0u32; MAX_INDICES];
        let mut offset: u32 = 0;
        for i in (0..MAX_INDICES).step_by(6) {
            quad_indices[i + 0] = offset + 0;
            quad_indices[i + 1] = offset + 1;
            quad_indices[i + 2] = offset + 2;
            quad_indices[i + 3] = offset + 2;
            quad_indices[i + 4] = offset + 3;
            quad_indices[i + 5] = offset + 0;
            offset += 4;
        }

        let quad_index_buffer = GLIndexBuffer::new(gl_rc.clone(), &quad_indices);
        quad_vertex_array.set_index_buffer(quad_index_buffer);

        let quad_vertices = [
            glm::Vec4::new(-0.5, -0.5, 0.0, 1.0),
            glm::Vec4::new(0.5, -0.5, 0.0, 1.0),
            glm::Vec4::new(0.5, 0.5, 0.0, 1.0),
            glm::Vec4::new(-0.5, 0.5, 0.0, 1.0),
        ];

        let quad_shader = GLShader::new(gl_rc.clone(), "quad_shader", VERTEX_SRC, FRAGMENT_SRC);

        let camera_uniform_buffer = GLUniformBuffer::new(
            gl_rc.clone(), 
            std::mem::size_of::<CameraData>(), 
            0);


        let data = Box::new(Renderer2DData {
            gl: gl_rc.clone(),
            quad_vertex_array: Box::new(quad_vertex_array),
            quad_vertex_buffer: Box::new(quad_buffer),
            quad_index_count: 0,
            quad_vertex_positions: quad_vertices,
            quad_vertex_buffer_idx: 0,
            quad_vertex_buffer_base: Box::new([QuadVertex::new(); MAX_VERTICES]),
            quad_shader,
            texture_slots: Vec::with_capacity(MAX_TEXTURE_SLOTS),
            texture_slot_idx: 0,
            camera_data: CameraData { view_projection: glm::Mat4::identity() },
            camera_uniform_buffer: Box::new(camera_uniform_buffer),
        });

        let ogl = OpenGLRendererAPI::new(gl_rc);
        ogl.set_viewport(0, 0, width, height);
        ogl.set_clear_color(&glm::Vec4::new(0.2, 0.5, 0.2, 1.0));
        let backend = RendererBackend::OpenGL(ogl);

        let stats = RenderStats::new();
        Self {
            data,
            backend,
            stats,
        }
    }

    pub fn print_stats(&self) {
        println!("{}", self.stats);
    }

    pub fn begin_scene(&mut self, camera: &OrthographicCamera) {
        self.clear_color();
        self.set_camera_data(camera);
        self.start_batch();
    }

    pub fn end_scene(&mut self) {
        self.flush();
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        match &self.backend {
            RendererBackend::OpenGL(ogl) => {
                ogl.set_viewport(0, 0, width, height);
            }
            _ => { },
        }
    }

    fn start_batch(&mut self) {
        self.data.quad_index_count = 0;
        self.data.quad_vertex_buffer_idx = 0;
        self.data.texture_slot_idx = 1;
    }

    fn next_batch(&mut self) {
        self.flush();
        self.start_batch();
    }

    fn flush(&mut self) {
        if self.data.quad_index_count > 0 {
            let bytes: &[u8] = to_bytes(
                &self.data.quad_vertex_buffer_base[0..self.data.quad_vertex_buffer_idx]
            );
            self.data.quad_vertex_buffer.set_data(bytes);

            for i in 0..self.data.texture_slots.len() {
                let tex = &self.data.texture_slots[i];
                tex.bind(i as u32);
            }
            
            self.data.quad_shader.bind();

            self.draw_indexed();
            self.stats.increment_draw_calls();
        }

        // TODO: circles
        // TODO: lines
        // TODO: text
    }

    pub fn draw_quad_ez(&mut self, position: &glm::Vec3, size: &glm::Vec2, color: glm::Vec4) {
        let mut identity = glm::Mat4::identity();
        let translate = glm::translate(&identity, position);
        let scale = glm::scale(&identity, &glm::vec2_to_vec3(size));
        let transform = translate * scale;
        self.draw_quad(&transform, color, -1);
    }

    pub fn draw_quad(&mut self, transform: &glm::Mat4, color: glm::Vec4, entity_id: i32) {
        if self.data.quad_index_count >= MAX_INDICES as u32 {
            self.next_batch();
        }

        for i in (0..QUAD_VERTEX_COUNT) {
            let vertex: &mut QuadVertex = &mut self.data.quad_vertex_buffer_base[
                self.data.quad_vertex_buffer_idx
            ];
            let position = &(transform * &self.data.quad_vertex_positions[i]);
            vertex.position = glm::vec4_to_vec3(
                position
            );
            vertex.color = color;
            vertex.tex_index = 0.0;
            vertex.tex_coord = TEXTURE_COORDS[i];
            vertex.tiling_factor = 1.0;
            vertex.entity_id = entity_id;
            self.data.quad_vertex_buffer_idx += 1;
        }
        self.data.quad_index_count += 6;
        self.stats.increment_quad_count();
    }

    pub fn draw_quad_texture(&mut self, transform: &glm::Mat4, texture: &GLTexture, tint_color: &glm::Vec4) {
        if self.data.quad_index_count >= MAX_INDICES as u32 {
            self.next_batch();
        }
        // TODO: potentially cache the texture?
        for i in (0..QUAD_VERTEX_COUNT) {
            let vertex: &mut QuadVertex = &mut self.data.quad_vertex_buffer_base[
                self.data.quad_vertex_buffer_idx
            ];
            let position = &(transform * &self.data.quad_vertex_positions[i]);
            vertex.position = glm::vec4_to_vec3(
                position
            );
            vertex.color = tint_color.clone();
            vertex.tex_coord = TEXTURE_COORDS[i];
            vertex.tex_index = 1.0;
            vertex.tiling_factor = 1.0;
            vertex.entity_id = -1;

            self.data.quad_vertex_buffer_idx += 1;
        }

        self.data.quad_index_count += 6;
        self.stats.increment_quad_count();
    }

    pub fn load_texture(&self, path: &str) -> Result<GLTexture, Box<dyn Error>> {
        todo!();
        // let texture = GLTexture::new(self.gl, path)
    }

    fn draw_indexed(&self) {
        match &self.backend {
            RendererBackend::OpenGL(opengl_api) => {
                opengl_api.draw_indexed(&*self.data.quad_vertex_array, self.data.quad_index_count as usize);
            },
            _ => { panic!("Unsupported renderer backend") },
        }
    }

    fn clear_color(&self) {
        match &self.backend {
            RendererBackend::OpenGL(ogl) => {
                ogl.set_clear_color(&glm::Vec4::new(0.2, 0.3, 0.5, 1.0));
            },
            _ => { },
        }
    }

    fn set_camera_data(&mut self, camera: &OrthographicCamera) {
        // TODO: get rid of this clone.
        self.data.camera_data.view_projection = camera.get_view_projection().clone();
        let bytes = self.data.camera_data.as_bytes();
        self.data.camera_uniform_buffer.set_data(
            &bytes,
            0);
        // self.data.camera_uniform_buffer.unbind();
    }

    pub fn get_error(&self) -> u32 {
        match &self.backend {
            RendererBackend::OpenGL(ogl) => {
                ogl.get_error()
            },
            _ => { 0 }
        }
    }

}

fn to_bytes(quad_vertices: &[QuadVertex]) -> &[u8] {

    println!("quad vertices size: {}", std::mem::size_of::<QuadVertex>() * quad_vertices.len());
    unsafe {
        std::slice::from_raw_parts(
            quad_vertices.as_ptr() as *const u8,
            std::mem::size_of::<QuadVertex>() * quad_vertices.len(),
        )
    }
}



impl RenderStats {
    fn new() -> Self {
        RenderStats {
            draw_calls: 0,
            quad_count: 0,
        }
    }

    fn increment_draw_calls(&mut self) {
        self.draw_calls += 1
    }

    fn increment_quad_count(&mut self) {
        self.quad_count += 1
    }

    fn total_vertex_count(&self) -> usize {
        self.quad_count * 4
    }

    fn total_index_count(&self) -> usize {
        self.quad_count * 6
    }
}



impl fmt::Display for RenderStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "draw calls: {}\nquad count:{}\ntotal vertices: {}\ntotal indices: {}", 
            self.draw_calls, self.quad_count, self.total_vertex_count(), self.total_index_count())
    }
}
