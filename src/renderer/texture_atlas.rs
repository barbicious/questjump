use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use glfw::Key::W;
use image::ColorType;
use rect_packer::Config;
use crate::registry::texture_registry::TextureRegistry;
use crate::renderer::bindable::Bindable;
use crate::renderer::raw_texture::image_texture::ImageTexture;

pub struct TextureMetadata {
    pub names: HashMap<String, usize>,
    pub src_rects: Vec<rect_packer::Rect>,
}

pub struct TextureAtlas {
    image_texture: ImageTexture,
    texture_metadata: TextureMetadata
}

impl TextureAtlas {
    pub const TEXTURES_PATH: &'static str = "res/textures/";
    
    const WIDTH: i32 = 512;
    const HEIGHT: i32 = 512;

    pub fn new(gl: Rc<glow::Context>) -> anyhow::Result<Self> {
        let mut texture_metadata = TextureMetadata {
            names: HashMap::new(),
            src_rects: Vec::new(),
        };

        let mut images = Vec::new();

        let mut i = 0;

        for entry in std::fs::read_dir(Self::TEXTURES_PATH)? {
            let entry = entry?;
            let path = entry.path();

            texture_metadata.names.insert(path.file_stem().unwrap().to_str().unwrap().to_string(), i);

            let image = image::open(path)?;

            images.push(image);

            i += 1
        }

        let mut packer = rect_packer::Packer::new(Config {
            width: Self::WIDTH,
            height: Self::HEIGHT,
            border_padding: 1,
            rectangle_padding: 1,
        });

        for image in &images {
            texture_metadata.src_rects.push(packer.pack(image.width() as i32, image.height() as i32, false).ok_or(anyhow::anyhow!("Cannot pack rectangle"))?);
        }

        let mut buf = vec![0; (Self::WIDTH * Self::HEIGHT) as usize * 4];

        for i in 0..texture_metadata.src_rects.len() {
            let rect = &texture_metadata.src_rects[i];
            let image = images[i].as_bytes();

            println!("{rect:?}");

            for y in 0..rect.height {
                for x in 0..rect.width {
                    let buf_index = ((y + rect.y) * Self::WIDTH + (x + rect.x)) as usize * 4;
                    let img_index = (y * rect.width + x) as usize * 4;
                    
                    buf[buf_index] = image[img_index];
                    buf[buf_index + 1] = image[img_index + 1];
                    buf[buf_index + 2] = image[img_index + 2];
                    buf[buf_index + 3] = image[img_index + 3];
                }
            }
        }

        image::save_buffer("atlas.png", &buf, Self::WIDTH as u32, Self::HEIGHT as u32, ColorType::Rgba8)?;

        let image_texture = ImageTexture::new(gl, "atlas.png")?;

        Ok(Self { image_texture, texture_metadata })
    }

    pub fn texture_metadata(&self) -> &TextureMetadata {
        &self.texture_metadata
    }
}

impl Deref for TextureAtlas {
    type Target = ImageTexture;

    fn deref(&self) -> &Self::Target {
        &self.image_texture
    }
}