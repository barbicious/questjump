use std::ops::Deref;
use std::rc::Rc;
use glow::{HasContext, PixelUnpackData};
use crate::renderer::bindable::Bindable;
use crate::renderer::raw_texture::framebuffer::Framebuffer;
use crate::renderer::raw_texture::RawTexture;

pub struct RenderTexture {
    raw_texture: RawTexture,
    framebuffer: Framebuffer,
    width: u32,
    height: u32,
}

impl RenderTexture {
    pub fn new(gl: Rc<glow::Context>, width: u32, height: u32) -> anyhow::Result<Self> {
        let framebuffer = Framebuffer::new(gl.clone())?;

        let raw_texture = RawTexture::new(gl)?;
        raw_texture.bind();

        unsafe {
            raw_texture.gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, width as i32, height as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, PixelUnpackData::Slice(None));
            raw_texture.gl.generate_mipmap(glow::TEXTURE_2D);

            raw_texture.gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, Some(raw_texture.id), 0);
        }

        framebuffer.complete()?;

        Ok(Self { raw_texture, framebuffer, width, height })
    }

    pub fn framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }
}

impl Deref for RenderTexture {
    type Target = RawTexture;

    fn deref(&self) -> &Self::Target {
        &self.raw_texture
    }
}