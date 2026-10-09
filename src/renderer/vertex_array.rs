use crate::renderer::bindable::Bindable;
use glow::HasContext;
use std::rc::Rc;

pub struct VertexArray {
    gl: Rc<glow::Context>,
    id: glow::NativeVertexArray,
}

impl VertexArray {
    pub fn new(gl: Rc<glow::Context>) -> anyhow::Result<Self> {
        let id = unsafe { gl.create_vertex_array().map_err(anyhow::Error::msg)? };

        unsafe {
            gl.bind_vertex_array(Some(id));
        }

        Ok(Self {
            id,
            gl,
        })
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.id);
        }
    }
}

impl Bindable for VertexArray {
    fn bind(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.id));
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl.bind_vertex_array(None);
        }
    }
}
