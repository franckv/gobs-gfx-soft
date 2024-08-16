use anyhow::Result;

use gobs_core::{Color, ImageExtent2D};

use image::{ImageFormat, RgbaImage};

pub struct Image {
    extent: ImageExtent2D,
    data: Vec<Color>,
}

impl Image {
    #[tracing::instrument(level = "debug")]
    pub fn new(extent: ImageExtent2D, fill: Color) -> Self {
        let mut data = Vec::with_capacity((extent.width * extent.height) as usize);
        for _ in 0..extent.width * extent.height {
            data.push(fill);
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

    pub fn put_pixel(&mut self, x: u32, y: u32, c: Color) {
        assert!(x < self.extent.width);
        assert!(y < self.extent.height);

        let idx = y * self.extent.width + x;
        self.data[idx as usize] = c;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let idx = y * self.extent.width + x;
        self.data[idx as usize]
    }

    pub fn save(&self, file: &str) -> Result<()> {
        let mut img = RgbaImage::new(self.extent.width, self.extent.height);

        for x in 0..self.extent.width {
            for y in 0..self.extent.height {
                let c = self.get_pixel(x, y);
                img.put_pixel(x, y, c.into());
            }
        }

        img.save_with_format(file, ImageFormat::Tga)?;

        Ok(())
    }
}
