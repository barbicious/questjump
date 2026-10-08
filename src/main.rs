mod renderer;
mod window;

use glow::HasContext;
use crate::renderer::bindable::Bindable;
use crate::renderer::buffer::index_buffer::IndexBuffer;
use crate::renderer::raw_texture::image_texture::ImageTexture;
use crate::renderer::raw_texture::render_texture::RenderTexture;
use crate::renderer::shader::ShaderProps;
use crate::renderer::vertex_array::VertexArray;
use crate::window::Window;

fn main() -> anyhow::Result<()> {
    let (mut window, renderer) = Window::new("cobblejump", 1280, 720)?;

    let quad_shader = shader!(
        renderer.gl(),
        ShaderProps {
            src: include_str!("renderer/quad.vert"),
            r#type: glow::VERTEX_SHADER
        },
        ShaderProps {
            src: include_str!("renderer/quad.frag"),
            r#type: glow::FRAGMENT_SHADER
        }
    );

    let quad_vertices = [
        -1.0, -1.0,
        1.0, -1.0,
        1.0, 1.0,
        -1.0, 1.0,
    ];

    let quad_vao = VertexArray::new(renderer.gl())?;

    let indices = [0, 1, 2, 0, 2, 3];

    let quad_vbo = vbo!(renderer.gl(), &quad_vao, glow::STATIC_DRAW, &quad_vertices, 2)?;

    let quad_ibo = IndexBuffer::new(renderer.gl(), glow::STATIC_DRAW, &indices)?;

    let shader = shader!(
        renderer.gl(),
        ShaderProps {
            src: include_str!("renderer/sprite.vert"),
            r#type: glow::VERTEX_SHADER
        },
        ShaderProps {
            src: include_str!("renderer/sprite.frag"),
            r#type: glow::FRAGMENT_SHADER
        }
    );
    shader.bind();

    let image_texture = ImageTexture::new(renderer.gl(), "res/textures/player.png")?;

    let vao = VertexArray::new(renderer.gl())?;

    let vertices = [
        0.0, 0.0, 0.0, 1.0,
        128.0, 0.0, 1.0, 1.0,
        128.0, 32.0, 1.0, 0.0,
        0.0, 32.0, 0.0, 0.0
    ];

    let vbo = vbo!(renderer.gl(), &vao, glow::STATIC_DRAW, &vertices, 2, 2)?;

    let ibo = IndexBuffer::new(renderer.gl(), glow::STATIC_DRAW, &indices)?;

    let projection_matrix = nalgebra_glm::ortho(0.0, 256.0, 0.0, 180.0, 0.01, 1.0);
    shader.set_mat4("u_proj", &projection_matrix);

    let render_texture = RenderTexture::new(renderer.gl(), 256, 180)?;

    while !window.should_close() {
        renderer.viewport(0, 0, 256, 180);

        render_texture.framebuffer().bind();
        renderer.clear();
        shader.bind();
        vao.bind();
        image_texture.bind();
        renderer.draw_triangle(6);

        renderer.viewport(0, 0, 1280, 720);

        render_texture.framebuffer().unbind();
        renderer.clear();
        quad_shader.bind();
        quad_vao.bind();
        render_texture.bind();
        renderer.draw_triangle(6);

        window.splat();
    }

    Ok(())
}
