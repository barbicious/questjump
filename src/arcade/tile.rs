use serde::Deserialize;
use crate::registry::texture_registry::TextureRegistry;
use crate::renderer::Renderer;
use crate::renderer::rect::Rect;

pub struct Tile {
    texture_id: usize,
}

impl Tile {
    pub fn new(texture_registry: &TextureRegistry, tile_payload: TilePayload) -> Self {
        let texture_id = texture_registry[tile_payload.texture_name.as_str()];

        Self { texture_id }
    }

    pub fn blit(
        &self,
        renderer: &mut Renderer,
        texture_registry: &TextureRegistry,
        x: i32,
        y: i32,
    ) {
        renderer.blit(
            &texture_registry[self.texture_id],
            Rect::new(x as f32 * 16.0, y as f32 * 16.0, 16.0, 16.0),
            Rect::new(0.0, 0.0, 16.0, 16.0),
            0.0,
        );
    }
}

#[derive(Deserialize)]
pub struct TilePayload {
    pub name: String,
    pub texture_name: String,
}
