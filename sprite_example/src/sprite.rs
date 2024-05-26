use std::sync::Arc;
use glium::backend::Facade;

use glium::texture::{RawImage2d, SrgbTexture2d};

#[derive(Clone)]
pub struct Sprite {
    pub(crate) texture: Arc<SrgbTexture2d>,
    pub(crate) position: (f32, f32),
    pub(crate) size: (f32, f32),
}

impl Sprite {
    pub fn new(texture: Arc<SrgbTexture2d>, position: (f32, f32), size: (f32, f32)) -> Sprite {
        Sprite { texture, position, size }
    }

    /// 1대1 비율로 Sprite 크기를 조절합니다.
    pub fn with_fixed_ratio(mut self) -> Sprite {
        let max_dimension = self.size.0.max(self.size.1);
        self.size = (max_dimension, max_dimension);
        self
    }
}

pub fn load_sprite<F: Facade>(display: &F, file_path: &str, position: (f32, f32), size_percent: f32) -> Sprite {
    let image = image::open(file_path).unwrap().into_rgba8();
    let image_dimensions = image.dimensions();
    let raw_image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = SrgbTexture2d::new(display, raw_image).unwrap();

    let (original_width, original_height) = (image_dimensions.0 as f32, image_dimensions.1 as f32);
    let scale = size_percent / 100.0;
    let size = (original_width * scale, original_height * scale);

    Sprite::new(Arc::new(texture), position, size)
}
