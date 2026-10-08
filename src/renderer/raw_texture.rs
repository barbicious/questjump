pub mod image_texture;
pub mod render_texture;
pub mod framebuffer;

use std::rc::Rc;
use glow::HasContext;
use crate::renderer::bindable::Bindable;

pub struct RawTexture {
    gl: Rc<glow::Context>,
    id: glow::NativeTexture,
}

impl RawTexture {
    pub fn new(gl: Rc<glow::Context>) -> anyhow::Result<Self> {
        let id = unsafe { gl.create_texture().map_err(anyhow::Error::msg)? };
        
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(id));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
        }
        
        Ok(Self { id, gl })
    }
}

impl Drop for RawTexture {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.id);
        }
    }
}

impl Bindable for RawTexture {
    fn bind(&self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }
}