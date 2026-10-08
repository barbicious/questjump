pub mod vertex_buffer;
pub mod index_buffer;

use crate::renderer::bindable::Bindable;
use glow::HasContext;
use std::rc::Rc;

pub struct Buffer {
    gl: Rc<glow::Context>,
    id: glow::NativeBuffer,
    target: u32,
}

impl Buffer {
    pub fn new<T>(
        gl: Rc<glow::Context>,
        target: u32,
        usage: u32,
        data: &[T],
    ) -> anyhow::Result<Self>
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let id = unsafe { gl.create_buffer().map_err(anyhow::Error::msg)? };

        unsafe {
            gl.bind_buffer(target, Some(id));
            gl.buffer_data_u8_slice(target, bytemuck::cast_slice(data), usage);
        }

        Ok(Buffer { gl, id, target })
    }

    pub fn upload_data<T>(&self, offset: i32, data: &[T]) -> anyhow::Result<()>
    where
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        unsafe {
            self.gl
                .buffer_sub_data_u8_slice(self.target, offset, bytemuck::cast_slice(data));
        }

        Ok(())
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.id);
        }
    }
}

impl Bindable for Buffer {
    fn bind(&self) {
        unsafe {
            self.gl.bind_buffer(self.target, Some(self.id));
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl.bind_buffer(self.target, None);
        }
    }
}
