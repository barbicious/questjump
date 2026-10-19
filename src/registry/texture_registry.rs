use std::collections::HashMap;
use std::ops::Index;
use std::path::Path;
use image::{ColorType, GenericImageView};
use rect_packer::{Config, Rect};
use crate::renderer::raw_texture::image_texture::ImageTexture;
use crate::renderer::Texture2D;
use crate::renderer::texture_atlas::TextureAtlas;

pub struct TextureRegistry {
    names: HashMap<String, usize>,
    textures: Vec<Texture2D>
}

impl TextureRegistry {
    pub fn new(texture_atlas: &TextureAtlas) -> anyhow::Result<Self> {
        let mut names = texture_atlas.texture_metadata().names.clone();
        let mut textures = Vec::new();
        
        for i in 0..texture_atlas.texture_metadata().src_rects.len() {
            let rect = &texture_atlas.texture_metadata().src_rects[i];
            
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

impl Index<&str> for TextureRegistry {
    type Output = usize;
    
    fn index(&self, index: &str) -> &usize {
        &self.names[index]
    }
}