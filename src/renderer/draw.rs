use super::buffer::*;
use glow::{self, HasContext};
use nalgebra_glm as glm;
use std::rc::Rc;



pub trait RendererApi {
    fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32);
    fn set_clear_color(&self, color: &glm::Vec4);
    fn clear(&self);
    // fn draw_indexed(&self, vertex_array: &impl VertexArray, index_count: usize);
    fn draw_lines(&self, vertex_array: &impl VertexArray, vertex_count: usize);
    fn set_line_width(&self, width: f32);
}



pub struct OpenGLRendererAPI(Rc<glow::Context>);



impl OpenGLRendererAPI {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        Self(gl)
    }

    pub fn draw_indexed(&self, vertex_array: &GLVertexArray, index_count: usize) {
        vertex_array.bind();
        let count = if let Some(idx_buffer) = vertex_array.get_index_buffer() {
            idx_buffer.get_count()
        } else {
            0
        };

        unsafe {
            self.0.draw_elements(glow::TRIANGLES, count as i32, glow::UNSIGNED_INT, 0);
        }
    } 
}


impl RendererApi for OpenGLRendererAPI {
    fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            self.0.viewport(x, y, width, height);
        }
    }

    fn set_clear_color(&self, color: &glm::Vec4) {
        unsafe {
            self.0.clear_color(color[0], color[1], color[2], color[3]);
        }
    }

    fn clear(&self) {
        unsafe {
            self.0.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    fn draw_lines(&self, vertex_array: &impl VertexArray, vertex_count: usize) {
        vertex_array.bind();
        unsafe {
            self.0.draw_arrays(glow::TRIANGLES, 0, vertex_count as i32);
        }
    }

    fn set_line_width(&self, width: f32) {
        unsafe {
            self.0.line_width(width);
        }
    }
}




