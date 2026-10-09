use std::ops::Deref;
use std::rc::Rc;
use crate::renderer::buffer::Buffer;

pub struct IndexBuffer {
    buffer: Buffer
}

impl IndexBuffer {
    pub fn new(gl: Rc<glow::Context>, usage: u32, data: &[u32]) -> anyhow::Result<Self> {
        let buffer = Buffer::new(gl, glow::ELEMENT_ARRAY_BUFFER, usage, data)?;
        
        Ok(Self { buffer })
    }
}

impl Deref for IndexBuffer {
    type Target = Buffer;
    
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}