pub mod bindable;
pub mod buffer;
pub mod shader;
pub mod vertex_array;
pub mod raw_texture;
pub mod spritebatch;
pub mod rect;
pub mod blittable;
pub mod window;

use crate::renderer::bindable::Bindable;
use crate::renderer::blittable::Blittable;
use crate::renderer::buffer::index_buffer::IndexBuffer;
use crate::renderer::buffer::vertex_buffer::VertexBuffer;
use crate::renderer::raw_texture::image_texture::ImageTexture;
use crate::renderer::raw_texture::render_texture::RenderTexture;
use crate::renderer::rect::Rect;
use crate::renderer::shader::{Shader, ShaderProps};
use crate::renderer::spritebatch::Spritebatch;
use crate::renderer::vertex_array::VertexArray;
use crate::{shader, vbo};
use glow::HasContext;
use std::rc::Rc;
use window::Window;

pub type Texture2D = Rect<i32>;

pub struct Renderer {
    gl: Rc<glow::Context>,
    quad_vao: VertexArray,
    quad_vbo: VertexBuffer,
    quad_ibo: IndexBuffer,
    quad_shader: Shader,
    sprite_shader: Shader,
    render_texture: RenderTexture,
    image_texture: ImageTexture,
    spritebatches: Vec<Spritebatch>,
}

impl Renderer {
    pub fn new(gl: Rc<glow::Context>) -> anyhow::Result<Self> {
        let quad_shader = shader!(
            gl.clone(),
            ShaderProps {
                src: include_str!("renderer/quad.vert"),
                r#type: glow::VERTEX_SHADER
            },
            ShaderProps {
                src: include_str!("renderer/quad.frag"),
                r#type: glow::FRAGMENT_SHADER
            }
        );

        let quad_vertices = [
            -1.0, -1.0,
            1.0, -1.0,
            1.0, 1.0,
            -1.0, 1.0,
        ];

        let quad_vao = VertexArray::new(gl.clone())?;

        let indices = [0, 1, 2, 0, 2, 3];

        let quad_vbo = vbo!(gl.clone(), &quad_vao, glow::STATIC_DRAW, &quad_vertices, 2)?;

        let quad_ibo = IndexBuffer::new(gl.clone(), glow::STATIC_DRAW, &indices)?;

        let sprite_shader = shader!(
        gl.clone(),
        ShaderProps {
            src: include_str!("renderer/sprite.vert"),
            r#type: glow::VERTEX_SHADER
        },
        ShaderProps {
            src: include_str!("renderer/sprite.frag"),
            r#type: glow::FRAGMENT_SHADER
        }
    );
        sprite_shader.bind();

        let image_texture = ImageTexture::new(gl.clone(), "res/textures/player.png")?;

        let projection_matrix = nalgebra_glm::ortho(0.0, 256.0, 0.0, 180.0, 0.01, 1.0);

        sprite_shader.set_mat4("u_proj", &projection_matrix);

        let render_texture = RenderTexture::new(gl.clone(), 256, 180)?;

        unsafe {
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            gl.enable(glow::BLEND);

            gl.clear_color(0.45, 0.55, 0.7, 1.0);
        }

        Ok(Self { gl, quad_vao, quad_vbo, quad_ibo, quad_shader, sprite_shader, render_texture, image_texture, spritebatches: Vec::new() })
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

    pub fn flush(&self) {
        self.viewport(0, 0, 256, 180);
        self.render_texture.framebuffer().bind();
        self.clear();
        self.sprite_shader.bind();
        self.image_texture.bind();
    }

    pub fn display(&mut self, window: &Window) {
        for i in 0..self.spritebatches.len() {
            self.spritebatches[i].display().unwrap();
            self.draw_triangle(self.spritebatches[i].indices() as i32);
            self.spritebatches[i].reset_blits();
        }

        self.viewport(0, 0, window.width(), window.height());
        self.render_texture.framebuffer().unbind();
        self.clear();
        self.quad_shader.bind();
        self.quad_vao.bind();
        self.render_texture.bind();
        self.draw_triangle(6);
    }
}

impl Blittable for Renderer {
    fn blit(&mut self, dst: Rect<f32>, src: Rect<f32>, rotation: f32) {
        if self.spritebatches.is_empty() {
            self.spritebatches.push(Spritebatch::new(self).unwrap());
        }

        self.spritebatches.last_mut().unwrap().blit(dst, src, rotation);
    }
}