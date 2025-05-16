use std::vec;
use std::rc::Rc;

use glow::*;
use nalgebra_glm as glm;



pub trait VertexArray<V: VertexBuffer> {
    fn bind(&self);
    fn unbind(&self);
    fn add_vertex_buffer(&mut self, buffer: V);
    fn get_vertex_buffers(&self) -> &[V];
    fn set_index_buffer(&mut self);
    fn get_index_buffer(&self); // TODO: fix return type once I figure index buffers lol
}



pub trait VertexBuffer {
    fn bind(&mut self);
    fn unbind(&mut self);
    fn set_data(&mut self, bytes: &[u8]);
    fn get_layout(&self) -> &BufferLayout;
}




pub struct GLVertexArray {
    id: u32,
    vertex_buffer_index: u32,
    vertex_buffers: Vec<GLVertexBuffer>,
    index_buffer: i32, // TODO: WHAT IS THIS FOR?

}



pub struct GLVertexBuffer {
    gl: Rc<Context>,
    vbo: NativeBuffer,
    layout: BufferLayout
}


#[derive(Default)]
pub struct BufferLayout {
    stride: u32,
    elements: Vec<BufferElement>,
}


#[derive(Default)]
pub struct BufferElement {
    name: String,
    dtype: ShaderDataType,
    size: u32,
    offset: u32,
    normalized: bool,
}


#[derive(Default)]
pub enum ShaderDataType {
    #[default]
    None,
    Float,
    Float2,
    Float3,
    Float4,
    Mat3,
    Mat4,
    Int,
    Int2,
    Int3,
    Int4,
    Bool,
}



pub struct BufferLayoutBuilder {
    offset: u32,
    layout: BufferLayout,
}



impl ShaderDataType {
    fn size(&self) -> u32 {
        match self {
            ShaderDataType::Float       => { 4 },
            ShaderDataType::Float2      => { 4 * 2 },
            ShaderDataType::Float3      => { 4 * 3 },
            ShaderDataType::Float4      => { 4 * 4 },
            ShaderDataType::Mat3        => { 4 * 3 * 3 },
            ShaderDataType::Mat4        => { 4 * 4 * 4 },
            ShaderDataType::Int         => { 4 },
            ShaderDataType::Int2        => { 4 * 2 },
            ShaderDataType::Int3        => { 4 * 3 },
            ShaderDataType::Int4        => { 4 * 4 },
            _ => 0,
        }
    }
}



impl BufferElement {
    pub fn new(dtype: ShaderDataType, name: &str, normalized: bool) -> Self {
        let size = dtype.size();
        Self {
            name: name.into(),
            dtype,
            size,
            offset: 0,
            normalized,
        }
    }
}



impl BufferLayout {
    pub fn stride(&self) -> u32 {
        self.stride
    }

    pub fn elements(&self) -> &[BufferElement] {
        &self.elements
    }
}



impl BufferLayoutBuilder {
    pub fn new() -> Self {
        Self {
            offset: 0,
            layout: BufferLayout {
                stride: 0,
                elements: Vec::new(),
            }
        }
    }

    pub fn element(mut self, mut element: BufferElement) -> Self {
        element.offset = self.offset;
        self.offset += element.size;
        self.layout.elements.push(element);
        self
    }

    pub fn build(mut self) -> BufferLayout {
        self.layout.stride = self.offset;
        self.layout
    }
}



impl GLVertexBuffer {
    pub fn new(gl: Rc<Context>, layout: BufferLayout) -> Self {
        unsafe {
            let vbo = gl.create_buffer()
                .expect("Failed to create OpenGL Buffer");

            Self {
                gl,
                vbo,
                layout,
            }
        }
    }
}



impl VertexBuffer for GLVertexBuffer {
    fn bind(&mut self) {
        unsafe {
            self.gl.bind_buffer(ARRAY_BUFFER, Some(self.vbo));
        }
    }

    fn unbind(&mut self) {
        unsafe {
            self.gl.bind_buffer(ARRAY_BUFFER, None);
        }
    }

    fn set_data(&mut self, bytes: &[u8]) {
        unsafe {
            self.bind();
            self.gl.buffer_data_u8_slice(ARRAY_BUFFER, bytes, STATIC_DRAW);
        }
    }

    fn get_layout(&self) -> &BufferLayout {
        &self.layout
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_builder() {
        let buffer_layout = BufferLayoutBuilder::new()
            .element(BufferElement::new(ShaderDataType::Float2, "position", false))
            .element(BufferElement::new(ShaderDataType::Float3, "color", false))
            .build();

        assert_eq!(20, buffer_layout.stride);
        assert_eq!(2, buffer_layout.elements().len());
    }
}




