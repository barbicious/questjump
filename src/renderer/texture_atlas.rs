use std::ops::Deref;
use std::rc::Rc;
use image::ColorType;
use rect_packer::Config;
use crate::renderer::bindable::Bindable;
use crate::renderer::raw_texture::image_texture::ImageTexture;

pub struct TextureAtlas {
    image_texture: ImageTexture,
}

impl TextureAtlas {
    const PATH: &'static str = "res/textures/";
    
    pub fn new(gl: Rc<glow::Context>) -> anyhow::Result<Self> {
        let mut images = Vec::new();

        for entry in std::fs::read_dir(Self::PATH)? {
            let entry = entry?;
            let path = entry.path();

            let image = image::open(path)?;

            images.push(image);
        }

        let mut packer = rect_packer::Packer::new(Config {
            width: 256,
            height: 256,
            border_padding: 1,
            rectangle_padding: 1,
        });

        let mut rects = Vec::new();

        for image in &images {
            rects.push(packer.pack(image.width() as i32, image.height() as i32, false).ok_or(anyhow::anyhow!("Cannot pack rectangle"))?);
        }

        let mut buf = [0; (256 * 256) * 4];

        for i in 0..rects.len() {
            let rect = &rects[i];
            let image = images[i].as_bytes();

            for y in 0..rect.height {
                for x in 0..rect.width {
                    buf[((y + rect.y) * 256 + (x + rect.x)) as usize * 4] = image[(y * rect.width + x) as usize * 4];
                    buf[((y + rect.y) * 256 + (x + rect.x)) as usize * 4 + 1] = image[(y * rect.width + x) as usize * 4 + 1];
                    buf[((y + rect.y) * 256 + (x + rect.x)) as usize * 4 + 2] = image[(y * rect.width + x) as usize * 4 + 2];
                    buf[((y + rect.y) * 256 + (x + rect.x)) as usize * 4 + 3] = image[(y * rect.width + x) as usize * 4 + 3];
                }
            }
        }

        image::save_buffer("atlas.png", &buf, 256, 256, ColorType::Rgba8)?;
        
        let image_texture = ImageTexture::new(gl, "atlas.png")?;
        
        Ok(Self { image_texture })
    }
}

impl Deref for TextureAtlas {
    type Target = ImageTexture;

    fn deref(&self) -> &Self::Target {
        &self.image_texture
    }
}