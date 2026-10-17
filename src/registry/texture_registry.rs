use std::collections::HashMap;
use std::path::Path;
use crate::renderer::Texture2D;

pub struct TextureRegistry {
    names: HashMap<String, usize>,
    textures: Vec<Texture2D>
}

impl TextureRegistry {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let names = HashMap::new();
        let textures = Vec::new();

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            let image = image::open(path)?;


        }

        Ok(Self { names, textures })
    }
}