use std::collections::HashMap;
use std::ops::Index;
use std::path::Path;
use image::{ColorType, GenericImageView};
use rect_packer::{Config, Rect};
use crate::renderer::raw_texture::image_texture::ImageTexture;
use crate::renderer::Texture2D;

pub struct TextureRegistry {
    names: HashMap<String, usize>,
    textures: Vec<Texture2D>
}

impl TextureRegistry {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let names = HashMap::new();
        let mut textures = Vec::new();

        let mut images = Vec::new();

        for entry in std::fs::read_dir(path)? {
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

        for image in images {
            rects.push(packer.pack(image.width() as i32, image.height() as i32, false).ok_or(anyhow::anyhow!("Cannot pack rectangle"))?);
        }
        
        for rect in rects {
            textures.push(Texture2D::new(rect.x, rect.y, rect.width, rect.height))
        }
        
        Ok(Self { names, textures })
    }
}

impl Index<usize> for TextureRegistry {
    type Output = Texture2D;
    
    fn index(&self, index: usize) -> &Texture2D {
        &self.textures[index]
    }
}