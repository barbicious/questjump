pub mod bindable;
pub mod buffer;
pub mod shader;
pub mod vertex_array;
pub mod raw_texture;

use glow::HasContext;
use std::rc::Rc;

pub struct Renderer {
    gl: Rc<glow::Context>,
}

impl Renderer {
    pub fn new(gl: Rc<glow::Context>) -> Self {
        unsafe {
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            gl.enable(glow::BLEND);

            gl.clear_color(0.7, 0.6, 0.2, 1.0);
        }

        Self { gl }
    }

    pub fn clear(&self) {
        unsafe {
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            self.gl.viewport(x, y, width, height);
        }
    }

    pub fn gl(&self) -> Rc<glow::Context> {
        self.gl.clone()
    }

    pub fn draw_triangle(&self, count: i32) {
        unsafe {
            self.gl.draw_elements(glow::TRIANGLES, count, glow::UNSIGNED_INT, 0);
        }
    }
}
