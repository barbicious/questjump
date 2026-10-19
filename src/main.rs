mod renderer;
pub mod registry;
mod arcade;

use crate::renderer::bindable::Bindable;
use crate::renderer::blittable::Blittable;
use crate::renderer::rect::Rect;
use glow::HasContext;
use renderer::window::Window;
use crate::registry::texture_registry::TextureRegistry;

fn main() -> anyhow::Result<()> {
    let (mut window, mut renderer) = Window::new("cobblejump", 1280, 720)?;

    let texture_registry = TextureRegistry::new(renderer.texture_atlas())?;

    while !window.should_close() {
        renderer.flush();

        for y in 0..3 {
            for x in 0..30 {
                renderer.blit(&texture_registry[texture_registry["cobblestone"]], Rect::new(x as f32 * 16.0, y as f32 * 16.0, 16.0, 16.0), Rect::new(0.0, 0.0, 16.0, 16.0), 0.0);
            }
        }
        renderer.display(&window);

        window.splat();
    }

    Ok(())
}