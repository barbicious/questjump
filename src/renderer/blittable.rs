use crate::renderer::rect::Rect;

pub trait Blittable {
    fn blit(&mut self, dst: Rect<f32>, src: Rect<f32>, rotation: f32);
}