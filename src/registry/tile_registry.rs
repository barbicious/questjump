use std::collections::HashMap;
use crate::arcade::tile::{Tile, TilePayload};
use crate::impl_registry_indexers;
use crate::registry::texture_registry::TextureRegistry;

pub struct TileRegistry {
    names: HashMap<String, usize>,
    values: Vec<Tile>
}

impl TileRegistry {
    pub fn new(texture_registry: &TextureRegistry) -> anyhow::Result<Self> {
        let mut names = HashMap::new();
        let mut values = Vec::new();

        for entry in std::fs::read_dir("res/tiles")? {
            let entry = entry?;
            let path = entry.path();

            let name = path.file_stem().unwrap().to_str().unwrap().to_string();

            let payload: TilePayload = ron::de::from_reader(std::fs::File::open(path)?)?;
            
            let id = values.len();

            values.push(Tile::new(texture_registry, payload));
            
            names.insert(name, id);
        }

        Ok(Self {
            names,
            values,
        })
    }
}

impl_registry_indexers!(Tile, TileRegistry);