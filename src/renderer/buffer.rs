use std::vec;
use std::rc::Rc;

use glow::*;
use nalgebra_glm as glm;



pub trait VertexArray {
    type Item: VertexBuffer;
    type Item2: IndexBuffer;

    fn bind(&self);
    fn unbind(&self);
    fn add_vertex_buffer(&mut self, buffer: &mut Self::Item);
    fn get_vertex_buffers(&self) -> &[Self::Item];
    fn set_index_buffer(&mut self, buffer: Self::Item2);
    fn get_index_buffer(&self) -> &Option<Self::Item2>; 
}



pub trait VertexBuffer {
    fn bind(&self);
    fn unbind(&self);
    fn set_data(&mut self, bytes: &[u8]);
    fn get_layout(&self) -> &BufferLayout;
}



pub trait IndexBuffer {
    fn bind(&self);
    fn unbind(&self);
    fn get_count(&self) -> usize;
}



#[derive(Debug)]
pub struct GLVertexArray {
    gl: Rc<Context>,
    vao: NativeVertexArray,
    vertex_buffer_index: u32,
    vertex_buffers: Vec<GLVertexBuffer>, // UNUSED
    index_buffer: Option<GLIndexBuffer>,
}



#[derive(Debug)]
pub struct GLVertexBuffer {
    gl: Rc<Context>,
    vbo: NativeBuffer,
    layout: BufferLayout
}



#[derive(Debug)]
pub struct GLIndexBuffer {
    gl: Rc<glow::Context>,
    ibo: NativeBuffer,
    count: usize,
}



pub struct GLUniformBuffer {
    gl: Rc<glow::Context>,
    buffer: NativeBuffer,
}



#[derive(Default,Debug)]
pub struct BufferLayout {
    stride: u32,
    elements: Vec<BufferElement>,
}

struct BufferLayoutIterator<'a> {
    index: usize,
    layout: &'a BufferLayout,
}


#[derive(Default, Debug)]
pub struct BufferElement {
    name: String,
    dtype: ShaderDataType,
    size: u32,
    offset: u32,
    normalized: bool,
}


#[derive(Default, Debug)]
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

    fn gl_base_type(&self) -> u32 {
        match &self {
            Self::Float |
                Self::Float2 | 
                Self::Float3 | 
                Self::Float4 |
                Self::Mat3 |
                Self::Mat4 => { FLOAT },
            Self::Int |
                Self::Int2 | 
                Self::Int3 | 
                Self::Int4 => { INT },
            Self::Bool => { BOOL },
            _ => { 
                assert!(false, "Unknown shader data type");
                0
            }
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

    fn get_component_count(&self) -> i32 {
        match self.dtype {
            ShaderDataType::Float | 
                ShaderDataType::Int => { 1 },
            ShaderDataType::Float2 | 
                ShaderDataType::Int2 => { 2 },
            ShaderDataType::Float3 | 
                ShaderDataType::Int3 => { 3 },
            ShaderDataType::Float4 | 
                ShaderDataType::Int4 => { 4 },
            ShaderDataType::Mat3 => { 3 },
            ShaderDataType::Mat4 => { 4 },
            ShaderDataType::Bool => { 1 },
            _ => { 
                assert!(false, "Unknown shader data type"); 
                0
            }
        }
    }
}



impl BufferLayout {
    pub fn stride(&self) -> i32 {
        self.stride as i32
    }

    pub fn elements(&self) -> &[BufferElement] {
        &self.elements
    }

    pub fn iter(&self) -> BufferLayoutIterator {
        BufferLayoutIterator {
            layout: &self,
            index: 0
        }
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



impl<'a> Iterator for BufferLayoutIterator<'a> {
    type Item = &'a BufferElement;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.layout.elements.len() {
            let result = Some(&self.layout.elements[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}



impl GLVertexBuffer {
    pub fn new(gl: Rc<Context>, layout: BufferLayout) -> Self {
        let vbo = unsafe { gl.create_buffer()
            .expect("Failed to create OpenGL Buffer") };

        Self {
            gl,
            vbo,
            layout,
        }
    }
}



impl Drop for GLVertexBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.vbo);
        }
    }
}



impl VertexBuffer for GLVertexBuffer {
    fn bind(&self) {
        unsafe {
            self.gl.bind_buffer(ARRAY_BUFFER, Some(self.vbo));
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl.bind_buffer(ARRAY_BUFFER, None);
        }
    }

    fn set_data(&mut self, bytes: &[u8]) {
        unsafe {
            self.bind();
            self.gl.buffer_sub_data_u8_slice(ARRAY_BUFFER, 0, bytes);
        }
    }

    fn get_layout(&self) -> &BufferLayout {
        &self.layout
    }
}



impl GLVertexArray {
    pub fn new(gl: Rc<Context>) -> Self {
        let vao = unsafe { gl.create_vertex_array()
            .expect("Failed to create OpenGL vertex array") };

        Self {
            gl,
            vao,
            vertex_buffers: Vec::new(),
            vertex_buffer_index: 0,
            index_buffer: None,
        }
    }
}



impl Drop for GLVertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vao);
        }
    }
}


impl VertexArray for GLVertexArray {
    type Item = GLVertexBuffer;
    type Item2 = GLIndexBuffer;

    fn bind(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.vao));
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl.bind_vertex_array(None);
        }
    }

    fn add_vertex_buffer(&mut self, buffer: &mut GLVertexBuffer) {
        self.bind();
        buffer.bind();
        let layout = buffer.get_layout();
        assert!(layout.elements().len() > 0);
        for element in layout.iter() {
            match element.dtype {
                ShaderDataType::Float | 
                    ShaderDataType::Float2 | 
                    ShaderDataType::Float3 |
                    ShaderDataType::Float4 => {
                        unsafe {
                            self.gl.enable_vertex_attrib_array(self.vertex_buffer_index);
                            self.gl.vertex_attrib_pointer_f32(
                                self.vertex_buffer_index,
                                element.get_component_count(),
                                element.dtype.gl_base_type(),
                                element.normalized, 
                                layout.stride as i32,
                                element.offset as i32);
                            self.vertex_buffer_index += 1;
                        }
                    },

                ShaderDataType::Int |
                    ShaderDataType::Int2 | 
                    ShaderDataType::Int3 | 
                    ShaderDataType::Int4 | 
                    ShaderDataType::Bool => {
                        unsafe {
                            self.gl.enable_vertex_attrib_array(self.vertex_buffer_index);
                            self.gl.vertex_attrib_pointer_i32(
                                self.vertex_buffer_index,
                                element.get_component_count(),
                                element.dtype.gl_base_type(), 
                                layout.stride as i32,
                                element.offset as i32);
                            self.vertex_buffer_index += 1;
                        }
                    },

                ShaderDataType::Mat3 |
                    ShaderDataType::Mat4 => {
                        let count = element.get_component_count();
                        for i in 0..count {
                            unsafe {
                                self.gl.enable_vertex_attrib_array(self.vertex_buffer_index);
                                self.gl.vertex_attrib_pointer_f32(
                                    self.vertex_buffer_index,
                                    count,
                                    element.dtype.gl_base_type(),
                                    element.normalized, 
                                    layout.stride as i32,
                                    element.offset as i32 + (std::mem::size_of::<f32>() as i32 * i * count)
                                ); 
                                self.gl.vertex_attrib_divisor(self.vertex_buffer_index, 1);
                                self.vertex_buffer_index += 1;
                            }
                        }
                    },

                _ => { assert!(false, "Unknown shader data type") },
            }
        }
    }

    fn get_vertex_buffers(&self) -> &[GLVertexBuffer] {
        &self.vertex_buffers
    }

    fn set_index_buffer(&mut self, buffer: GLIndexBuffer) {
        self.bind();
        buffer.bind();

        self.index_buffer = Some(buffer);
    }

    fn get_index_buffer(&self) -> &Option<GLIndexBuffer> {
        &self.index_buffer
    }
}



impl GLIndexBuffer {
    pub fn new(gl: Rc<glow::Context>, indices: &[u32]) -> Self {
        unsafe {
            let ibo = gl.create_buffer()
                .expect("Failed to create opengl index buffer.");
            gl.bind_buffer(ARRAY_BUFFER, Some(ibo));

            let bytes: &[u8] = core::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * core::mem::size_of::<u32>(),
            );
            gl.buffer_data_u8_slice(ARRAY_BUFFER, bytes, STATIC_DRAW);

            gl.bind_buffer(ARRAY_BUFFER, None);
            
            Self {
                gl,
                ibo,
                count: indices.len(),
            }
        }
    }
}



impl IndexBuffer for GLIndexBuffer {
    fn bind(&self) {
        unsafe {
            self.gl.bind_buffer(ARRAY_BUFFER, Some(self.ibo));
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl.bind_buffer(ARRAY_BUFFER, None);
        }
    }

    fn get_count(&self) -> usize {
        self.count
    }
}



impl Drop for GLIndexBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.ibo);
        }
    }
}



impl GLUniformBuffer {
    pub fn new(gl: Rc<glow::Context>, size: i32, binding: u32) -> Self {
        unsafe {
            let buffer = gl.create_buffer()
                .expect("Failed to create opengl buffer");
            gl.named_buffer_data_size(buffer, size, DYNAMIC_DRAW);
            gl.bind_buffer_base(UNIFORM_BUFFER, binding, Some(buffer));
            Self {
                gl,
                buffer,
            }
        }
    }

    pub fn set_data(&mut self, bytes: &[u8], offset: usize) {
        unsafe {
            self.gl.named_buffer_sub_data_u8_slice(self.buffer, offset as i32, bytes);
        }
    }
}



impl Drop for GLUniformBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.buffer);
        }
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




