mod buffer;
use buffer::*;

mod draw;
use draw::*;

mod shader;
use shader::*;

use nalgebra_glm as glm;
use glow;

use std::rc::Rc;
use std::fmt;



pub enum RendererBackend {
    OpenGL(OpenGLRendererAPI),
    None,
}



#[derive(Copy, Clone)]
struct QuadVertex {
    position: glm::Vec2,
    color: glm::Vec3,
    entity_id: i64,
    // TODO: implement textures...
    // tex_coord: glm::Vec2,
    // tex_index: f32,
    // tiling_factor: f32,
}



impl QuadVertex {
    fn new() -> Self {
        Self {
            position: glm::Vec2::zeros(),
            color: glm::Vec3::zeros(),
            entity_id: -1,
        }
    }
}



const MAX_QUADS: usize = 20000;
const MAX_VERTICES: usize = MAX_QUADS * 4;
const MAX_INDICES: usize = MAX_QUADS * 6;
const MAX_TEXTURE_SLOTS: usize = 32;

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
    quad_vertex_buffer_base: [QuadVertex; MAX_VERTICES],
    quad_vertex_buffer_idx: usize, // index position of current quad_vertex_buffer_base

    quad_vertex_positions: [glm::Vec4; 4], 
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
    pub fn new(gl: glow::Context) -> Self {
        let quad_layout = BufferLayoutBuilder::new()
            .element(BufferElement::new(ShaderDataType::Float3, "a_Position", false))
            .element(BufferElement::new(ShaderDataType::Float4, "a_Color", false))
            .element(BufferElement::new(ShaderDataType::Int, "a_EntityId", false))
            .build();

        let gl_rc = Rc::new(gl);
        let mut quad_buffer = GLVertexBuffer::new(gl_rc.clone(), quad_layout);
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

        let data = Box::new(Renderer2DData {
            gl: gl_rc.clone(),
            quad_vertex_array: Box::new(quad_vertex_array),
            quad_vertex_buffer: Box::new(quad_buffer),
            quad_index_count: 0,
            quad_vertex_positions: quad_vertices,
            quad_vertex_buffer_idx: 0,
            quad_vertex_buffer_base: [QuadVertex::new(); MAX_VERTICES],
        });
        let backend = RendererBackend::OpenGL(OpenGLRendererAPI::new(gl_rc));
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

    pub fn begin_scene(&mut self) {
        // TODO: setup camera?
        //
        self.start_batch();
    }

    pub fn end_scene(&mut self) {
        self.flush();
    }

    fn start_batch(&mut self) {
        self.data.quad_index_count = 0;
        self.data.quad_vertex_buffer_idx = 0;
    }

    fn next_batch(&mut self) {
        self.flush();
        self.start_batch();
    }

    fn flush(&mut self) {
        if self.data.quad_index_count > 0 {
            unsafe {
                let bytes: &[u8] = core::slice::from_raw_parts(
                    self.data.quad_vertex_buffer_base[0..self.data.quad_vertex_buffer_idx].as_ptr() as *const u8,
                    self.data.quad_vertex_buffer_idx * core::mem::size_of::<QuadVertex>()
                );
                self.data.quad_vertex_buffer.set_data(bytes);
            }

            // TODO: bind textures here
            
            // TODO: Bind shader?

            self.draw_indexed();
            self.stats.increment_draw_calls();
        }

        // TODO: circles
        // TODO: lines
        // TODO: text
    }

    pub fn draw_quad(&mut self) {

    }

    fn draw_indexed(&self) {
        match &self.backend {
            RendererBackend::OpenGL(opengl_api) => {
                opengl_api.draw_indexed(&*self.data.quad_vertex_array, self.data.quad_index_count as usize);
            },
            _ => { panic!("Unsupported renderer backend") },
        }
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
