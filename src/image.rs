use std::fs::File;
use std::io::Write;

use crate::helpers::Color;

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    image_data: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize, image_data: Vec<Color>) -> Self {
        assert!(width * height == image_data.len());
        Self {
            width,
            height,
            image_data,
        }
    }

    fn tone_map(&self) -> Vec<u8> {
        let mut pixel_data = Vec::with_capacity(self.width * self.height);

        for j in 0..self.height {
            for i in 0..self.width {
                let index = (j * self.width) + i;
                let pixel = self.image_data[index];
                let pixel_color = [
                    (pixel.x.min(1.0) * 255.0) as u8,
                    (pixel.y.min(1.0) * 255.0) as u8,
                    (pixel.z.min(1.0) * 255.0) as u8,
                ];
                pixel_data.extend_from_slice(&pixel_color);
            }
        }
        pixel_data
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let data = self.tone_map();

        let mut file = File::create(path)?;

        let _ = file.write(std::format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes())?;

        let _ = file.write(data.as_slice())?;
        Ok(())
    }
}
