use anyhow::Result;
use image::{ImageFormat, RgbaImage};

use gobs_core::{Color, ImageExtent2D};

pub struct Depth {
    extent: ImageExtent2D,
    data: Vec<f32>,
}

impl Depth {
    #[tracing::instrument(level = "debug")]
    pub fn new(extent: ImageExtent2D) -> Self {
        let mut data = Vec::with_capacity((extent.width * extent.height) as usize);
        for _ in 0..extent.width * extent.height {
            data.push(f32::MIN);
        }
        Self { extent, data }
    }

    pub fn extent(&self) -> ImageExtent2D {
        self.extent
    }

    pub fn width(&self) -> u32 {
        self.extent.width
    }

    pub fn height(&self) -> u32 {
        self.extent.height
    }

    pub fn depth_test(&mut self, x: u32, y: u32, depth: f32) -> bool {
        if depth > self.get_pixel(x, y) {
            self.put_pixel(x, y, depth);
            true
        } else {
            false
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, d: f32) {
        assert!(x < self.extent.width);
        assert!(y < self.extent.height);

        let idx = y * self.extent.width + x;
        self.data[idx as usize] = d;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> f32 {
        let idx = y * self.extent.width + x;
        self.data[idx as usize]
    }

    pub fn save(&self, file: &str) -> Result<()> {
        let mut img = RgbaImage::new(self.extent.width, self.extent.height);

        for x in 0..self.extent.width {
            for y in 0..self.extent.height {
                let d = self.get_pixel(x, y);
                if d > 0. {
                    img.put_pixel(x, y, (Color::WHITE * d).into());
                }
            }
        }

        img.save_with_format(file, ImageFormat::Tga)?;

        Ok(())
    }
}
