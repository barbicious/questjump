use crate::renderer::bindable::Bindable;
use crate::renderer::buffer::Buffer;
use crate::renderer::vertex_array::VertexArray;
use glow::HasContext;
use std::ops::Deref;
use std::rc::Rc;

pub struct VertexBuffer {
    buffer: Buffer,
}

impl VertexBuffer {
    pub fn new(gl: Rc<glow::Context>, usage: u32, data: &[f32]) -> anyhow::Result<Self> {
        let buffer = Buffer::new(gl, glow::ARRAY_BUFFER, usage, data)?;

        Ok(Self { buffer })
    }

    pub fn attrib(
        &self,
        vertex_array: &VertexArray,
        index: u32,
        size: i32,
        stride: i32,
        offset: i32,
    ) {
        vertex_array.bind();

        unsafe {
            self.gl.vertex_attrib_pointer_f32(
                index,
                size,
                glow::FLOAT,
                false,
                stride * size_of::<f32>() as i32,
                offset * size_of::<f32>() as i32,
            );
            self.gl.enable_vertex_attrib_array(index);
        }
    }
}

impl Deref for VertexBuffer {
    type Target = Buffer;

    fn deref(&self) -> &Buffer {
        &self.buffer
    }
}

#[macro_export]
macro_rules! vbo {
    ($gl:expr, $vao:expr, $usage:expr, $data:expr, $($size:expr),+) => {{
            use crate::renderer::buffer::vertex_buffer::VertexBuffer;

            VertexBuffer::new($gl, $usage, $data).map(|vbo| {
                let stride = 0 $( + $size)+;
    
                #[allow(unused_assignments)]
                {
                    let mut index = 0;
                    let mut offset = 0;
    
                    $(
                        vbo.attrib($vao, index, $size, stride, offset);
    
                        offset += $size;
                        index += 1;
                    )+
                }
    
                vbo
            })
    }};
}
