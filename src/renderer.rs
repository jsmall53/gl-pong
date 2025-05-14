use std::vec;

use glow::*;
use nalgebra_glm as glm;

struct VertexArray {
    id: u32,
    vertex_buffer_index: u32,
    vertex_buffers: Vec<VertexBuffer>,
    index_buffer: i32, // TODO: WHAT IS THIS FOR?

}

struct VertexBuffer {
    id: NativeBuffer,
    layout: BufferLayout
}

struct BufferLayout {
    stride: u32,
    elements: Vec<BufferElement>,
}

struct BufferElement {
    name: String,
    data_type: ShaderDataType,
    size: u32,
    offset: usize,
    normalized: bool,
}

enum ShaderDataType {
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


impl ShaderDataType {
    fn size(&self) -> usize {
        0usize
        // match self {
        //     Float() => { 4 },
        //     Float2 => { 4 * 2 },
        //     Float3 => { 4 * 3 },
        //     Float4 => { 4 * 4 },
        // }
    }
}


impl BufferElement {

}


impl BufferLayout {
    fn new() -> Self {
        BufferLayout {
            stride: 0u32,
            elements: Vec::new(),
        }
    }

    fn from_elements(elements: Vec<BufferElement>) -> Self {
        let mut stride: u32 = 0;

        for element in &elements {
            stride += element.size;
        }

        BufferLayout {
            stride: 0u32,
            elements,
        }
    }

    pub fn stride(&self) -> u32 {
        self.stride
    }

    pub fn elements(&self) -> &[BufferElement] {
        &self.elements
    }
}


impl VertexBuffer {
    pub fn new(gl: &Context, size: u32, vertices: &[f32]) -> Self {
        unsafe {
           match gl.create_buffer() {
               Err(e) => { panic!("FATAL: Failed to create vertex buffer.") },
               Ok(id) => {

                   let bytes: &[u8] = core::slice::from_raw_parts(
                       vertices.as_ptr() as *const u8,
                       vertices.len() * core::mem::size_of::<f32>()
                   );

                   gl.bind_buffer(ARRAY_BUFFER, Some(id));
                   gl.buffer_data_u8_slice(ARRAY_BUFFER, bytes, STATIC_DRAW);

                   VertexBuffer {
                       id,
                       layout: BufferLayout::new(),
                   }

               }
           }
        }
    }
}


