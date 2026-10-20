use crate::renderer::Texture2D;
use crate::renderer::raw_texture::image_texture::ImageTexture;
use crate::renderer::texture_atlas::TextureAtlas;
use image::{ColorType, GenericImageView};
use rect_packer::{Config, Rect};
use std::collections::HashMap;
use std::path::Path;
use crate::impl_registry_indexers;

pub struct TextureRegistry {
    names: HashMap<String, usize>,
    values: Vec<Texture2D>,
}

impl TextureRegistry {
    pub fn new(texture_atlas: &TextureAtlas) -> anyhow::Result<Self> {
        let mut names = texture_atlas.texture_metadata().names.clone();
        let mut values = Vec::new();

        for i in 0..texture_atlas.texture_metadata().src_rects.len() {
            let rect = &texture_atlas.texture_metadata().src_rects[i];

            values.push(Texture2D::new(rect.x, rect.y, rect.width, rect.height))
        }

        Ok(Self { names, values })
    }
}

impl_registry_indexers!(Texture2D, TextureRegistry);