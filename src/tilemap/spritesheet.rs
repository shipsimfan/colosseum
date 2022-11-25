use crate::{Texture, Window};
use alexandria::{Input, SampleType};
use ginger::{Image, Pixel};

pub struct Spritesheet {
    width: usize,
    key: u16,
    image: Vec<Pixel<f32>>,
}

impl Spritesheet {
    pub fn new(width: usize) -> Self {
        Spritesheet {
            width,
            key: 1,
            image: vec![Pixel::newa(0.0, 0.0, 0.0, 0.0); width * width],
        }
    }

    pub fn with_capacity(width: usize, capacity: usize) -> Self {
        let mut image = Vec::with_capacity((capacity + 1) * width * width);
        for _ in 0..width * width {
            image.push(Pixel::newa(0.0, 0.0, 0.0, 0.0))
        }

        Spritesheet {
            width,
            key: 1,
            image,
        }
    }

    pub fn add_image(&mut self, image: &Image<f32>) -> u16 {
        let ret = self.key;
        self.key += 1;

        assert!(image.width() == self.width);
        assert!(image.height() == self.width);
        self.image
            .extend(image.pixels().iter().map(|pixel| pixel.clone()));

        ret
    }

    pub fn next_key(&self) -> u16 {
        self.key
    }

    pub fn create_texture<I: Input>(
        self,
        sample_type: SampleType,
        window: &mut Window<I>,
    ) -> Texture {
        Texture::new(
            Image::from_vec(self.width, self.image.len() / self.width, self.image),
            sample_type,
            window,
        )
    }
}
