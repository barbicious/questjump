use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;
use glow::{HasContext, PixelUnpackData};
use crate::renderer::raw_texture::RawTexture;

pub struct ImageTexture {
    raw_texture: RawTexture,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(gl: Rc<glow::Context>, path: P) -> anyhow::Result<Self> {
        let raw_texture = RawTexture::new(gl)?;

        let image = image::open(path)?;

        let width = image.width();
        let height = image.height();

        let data = image.as_bytes();

        unsafe {
            raw_texture.gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, width as i32, height as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, PixelUnpackData::Slice(Some(data)));
            raw_texture.gl.generate_mipmap(glow::TEXTURE_2D);
        }

        Ok(Self { raw_texture, width, height })
    }
}


impl Deref for ImageTexture {
    type Target = RawTexture;
    
    fn deref(&self) -> &Self::Target {
        &self.raw_texture
    }
}