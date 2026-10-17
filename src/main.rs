mod renderer;
pub mod registry;

use crate::renderer::bindable::Bindable;
use crate::renderer::blittable::Blittable;
use crate::renderer::rect::Rect;
use glow::HasContext;
use renderer::window::Window;
use crate::registry::texture_registry::TextureRegistry;

fn main() -> anyhow::Result<()> {
    let (mut window, mut renderer) = Window::new("cobblejump", 1280, 720)?;

    let texture_registry = TextureRegistry::new("res/textures")?;

    while !window.should_close() {
        renderer.flush();

        renderer.blit(&texture_registry[1], Rect::new(0.0, 0.0, 128.0, 32.0), Rect::new(0.0, 0.0, 128.0, 32.0), 0.0);

        renderer.display(&window);

        window.splat();
    }

    Ok(())
}