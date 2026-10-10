use crate::renderer::bindable::Bindable;
use crate::renderer::buffer::index_buffer::IndexBuffer;
use crate::renderer::buffer::vertex_buffer::VertexBuffer;
use crate::renderer::rect::Rect;
use crate::renderer::Renderer;
use crate::renderer::vertex_array::VertexArray;
use crate::vbo;

pub struct Spritebatch {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    blits: u32,
    vertex_array: VertexArray,
    index_buffer: IndexBuffer,
    vertex_buffer: VertexBuffer,
}

impl Spritebatch {
    pub fn new(renderer: &Renderer) -> anyhow::Result<Self> {
        let indices = vec![0; 1024 * 6];

        let vertices = vec![0.0; 1024 * 16];

        let vertex_array = VertexArray::new(renderer.gl())?;

        let vertex_buffer = vbo!(renderer.gl(), &vertex_array, glow::DYNAMIC_DRAW, vertices.as_slice(), 2, 2)?;

        let index_buffer = IndexBuffer::new(renderer.gl(), glow::DYNAMIC_DRAW, indices.as_slice())?;

        Ok(Self {
            vertices,
            indices,
            blits: 0,
            vertex_array,
            vertex_buffer,
            index_buffer,
        })

    }

    pub fn blit(&mut self, dst: Rect<f32>, src: Rect<f32>, rotation: f32) {
        let origin = nalgebra_glm::vec2(dst.x, dst.y);

        let cos_rotation = rotation.to_radians().cos();
        let sin_rotation = rotation.to_radians().sin();

        let rotation_matrix = nalgebra_glm::Mat2::new(cos_rotation, sin_rotation, -sin_rotation, cos_rotation);

        let mut positions = [
            nalgebra_glm::vec2(dst.x, dst.y),
            nalgebra_glm::vec2(dst.x + dst.w, dst.y),
            nalgebra_glm::vec2(dst.x + dst.w, dst.y + dst.h),
            nalgebra_glm::vec2(dst.x, dst.y + dst.h),
        ];

        for pos in &mut positions {
            let relative = *pos - origin;
            *pos =  origin + (rotation_matrix * relative);
        }

        let indices = [0 + self.blits * 4, 1 + self.blits * 4, 2 + self.blits * 4, 0 + self.blits * 4, 2 + self.blits * 4, 3 + self.blits * 4];

        let vertices = [
            positions[0].x, positions[0].y, src.x / 128.0, src.y / 32.0 + src.h / 32.0,
            positions[1].x, positions[1].y, src.x / 128.0 + src.w / 128.0, src.y / 32.0 + src.h / 32.0,
            positions[2].x, positions[2].y, src.x / 128.0 + src.w / 128.0, src.y / 32.0,
            positions[3].x, positions[3].y, src.x / 128.0, src.y / 32.0
        ];


        self.indices[(self.blits as usize * 6)..(self.blits as usize * 6 + 6)].copy_from_slice(indices.as_slice());
        self.vertices[(self.blits as usize * 16)..(self.blits as usize * 16 + 16)].copy_from_slice(vertices.as_slice());

        self.blits += 1;
    }

    pub fn display(&mut self, renderer: &Renderer) -> anyhow::Result<()> {
        self.vertex_array.bind();

        self.vertex_buffer.upload_data(0, self.vertices.as_slice())?;
        self.index_buffer.upload_data(0, self.indices.as_slice())?;

        renderer.draw_triangle(12);

        Ok(())
    }
}