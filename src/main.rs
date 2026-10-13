mod renderer;
mod window;

use glow::HasContext;
use crate::renderer::bindable::Bindable;
use crate::renderer::blittable::Blittable;
use crate::renderer::buffer::index_buffer::IndexBuffer;
use crate::renderer::raw_texture::image_texture::ImageTexture;
use crate::renderer::raw_texture::render_texture::RenderTexture;
use crate::renderer::rect::Rect;
use crate::renderer::shader::ShaderProps;
use crate::renderer::spritebatch::Spritebatch;
use crate::renderer::vertex_array::VertexArray;
use crate::window::Window;

fn main() -> anyhow::Result<()> {
    let (mut window, mut renderer) = Window::new("cobblejump", 1280, 720)?;

    while !window.should_close() {
        renderer.flush();

        renderer.blit(Rect::new(100.0, 80.0, 32.0, 32.0), Rect::new(0.0, 0.0, 32.0, 32.0), 21.0);
        renderer.blit(Rect::new(30.0, 120.0, 64.0, 32.0), Rect::new(32.0, 0.0, 64.0, 32.0), 4.0);

        renderer.display(&window);

        window.splat();
    }

    Ok(())
}
