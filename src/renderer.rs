use std::vec;

use glow::*;
use nalgebra_glm as glm;

pub struct VertexArray {
    id: u32,
    vertex_buffer_index: u32,
    vertex_buffers: Vec<VertexBuffer>,
    index_buffer: i32, // TODO: WHAT IS THIS FOR?

}

pub struct VertexBuffer {
    id: NativeBuffer,
    layout: BufferLayout
}

pub struct BufferLayout {
    stride: u32,
    elements: Vec<BufferElement>,
}

pub struct BufferElement {
    name: String,
    dtype: ShaderDataType,
    size: u32,
    offset: u32,
    normalized: bool,
}

pub enum ShaderDataType {
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


// impl VertexBuffer {
//     pub fn new(gl: &Context, size: u32, vertices: &[f32]) -> Self {
//         unsafe {
//            match gl.create_buffer() {
//                Err(e) => { panic!("FATAL: Failed to create vertex buffer.") },
//                Ok(id) => {
//
//                    let bytes: &[u8] = core::slice::from_raw_parts(
//                        vertices.as_ptr() as *const u8,
//                        vertices.len() * core::mem::size_of::<f32>()
//                    );
//
//                    gl.bind_buffer(ARRAY_BUFFER, Some(id));
//                    gl.buffer_data_u8_slice(ARRAY_BUFFER, bytes, STATIC_DRAW);
//
//                    VertexBuffer {
//                        id,
//                        layout: BufferLayout::new(),
//                    }
//
//                }
//            }
//         }
//     }
// }

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
