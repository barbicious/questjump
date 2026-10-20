mod arcade;
pub mod registry;
mod renderer;

use crate::registry::texture_registry::TextureRegistry;
use crate::renderer::bindable::Bindable;
use crate::renderer::blittable::Blittable;
use crate::renderer::rect::Rect;
use glow::HasContext;
use renderer::window::Window;
use crate::registry::tile_registry::TileRegistry;

fn main() -> anyhow::Result<()> {
    let (mut window, mut renderer) = Window::new("cobblejump", 1280, 720)?;

    let texture_registry = TextureRegistry::new(renderer.texture_atlas())?;
    let tile_registry = TileRegistry::new(&texture_registry)?;

    while !window.should_close() {
        renderer.flush();

        for y in 0..3 {
            for x in 0..30 {
                tile_registry[tile_registry["cobblestone"]].blit(&mut renderer, &texture_registry, x, y)
            }
        }

        renderer.display(&window);

        window.splat();
    }

    Ok(())
}
