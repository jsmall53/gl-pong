use super::buffer::*;
use glow::{self, HasContext, TRIANGLES};
use std::rc::Rc;



struct OpenGLRendererAPI;



impl OpenGLRendererAPI {
    fn draw(gl: Rc<glow::Context>, vertex_array: &GLVertexArray) {
        unsafe {
            vertex_array.bind();
            gl.draw_arrays(TRIANGLES, 0, 6);
        }
    }
}





