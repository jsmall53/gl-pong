pub mod buffer;
use buffer::*;

mod draw;
use draw::*;

use nalgebra_glm as glm;
use glow;
use std::rc::Rc;




struct PongRenderer {
    gl: Rc<glow::Context>,

    width: i32, 
    height: i32,


}



struct PaddleRenderer {
    gl: Rc<glow::Context>,
}



const MAX_QUADS: usize = 20000;
const MAX_VERTICES: usize = MAX_QUADS * 4;
const MAX_INDICES: usize = MAX_QUADS * 6;
const MAX_TEXTURE_SLOTS: usize = 32;

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



struct Renderer2DData {
    gl: Rc<glow::Context>,
    
    quad_vertex_array: Box<dyn VertexArray<Item = GLVertexBuffer, Item2 = GLIndexBuffer>>,
    quad_vertex_buffer: Box<dyn VertexBuffer>,
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

    quad_vertex_positions: [glm::Vec4; 6], //  TODO; update once index buffers
}



struct Renderer2D {
    data: Renderer2DData,
}



impl Renderer2D {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        let quad_layout = BufferLayoutBuilder::new()
            .element(BufferElement::new(ShaderDataType::Float3, "a_Position", false))
            .element(BufferElement::new(ShaderDataType::Float4, "a_Color", false))
            .element(BufferElement::new(ShaderDataType::Int, "a_EntityId", false))
            .build();
        let mut quad_buffer = GLVertexBuffer::new(gl.clone(), quad_layout);
        let mut quad_vertex_array = GLVertexArray::new(gl.clone());
        quad_vertex_array.add_vertex_buffer(&mut quad_buffer);
        // TODO: update once index buffers
        let quad_vertices = [
            glm::Vec4::new(-0.5, -0.5, 0.0, 1.0),
            glm::Vec4::new(0.5, -0.5, 0.0, 1.0),
            glm::Vec4::new(0.5, 0.5, 0.0, 1.0),

            glm::Vec4::new(-0.5, -0.5, 0.0, 1.0),
            glm::Vec4::new(-0.5, 0.5, 0.0, 1.0),
            glm::Vec4::new(0.5, 0.5, 0.0, 1.0),
        ];

        let data = Renderer2DData {
            gl: gl.clone(),
            quad_vertex_array: Box::new(quad_vertex_array),
            quad_vertex_buffer: Box::new(quad_buffer),
            quad_index_count: 0,
            quad_vertex_positions: quad_vertices,
            quad_vertex_buffer_idx: 0,
            quad_vertex_buffer_base: [QuadVertex::new(); MAX_VERTICES],
        };

        Self {
            data,
        }
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
            
        }
    }

    pub fn draw_quad(&mut self) {

    }
}



