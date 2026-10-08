use std::rc::Rc;
use glow::HasContext;
use crate::renderer::bindable::Bindable;

pub struct Framebuffer {
    gl: Rc<glow::Context>,
    id: glow::NativeFramebuffer,
}

impl Framebuffer {
    pub fn new(gl: Rc<glow::Context>) -> anyhow::Result<Self> {
        let id = unsafe {
            gl.create_framebuffer().map_err(anyhow::Error::msg)?
        };
        
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(id));
        }

        Ok(Self { gl, id })
    }
    
    pub fn complete(&self) -> anyhow::Result<()> {
        unsafe {
            if self.gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
                self.gl.delete_framebuffer(self.id);

                anyhow::bail!("Failed to create framebuffer")
            }
            
            Ok(())
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_framebuffer(self.id);
        }
    }
}

impl Bindable for Framebuffer {
    fn bind(&self) {
        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
        }
    }
    
    fn unbind(&self) {
        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }
}