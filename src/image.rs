use std::io::BufWriter;
use std::{fs::File, path::Path};

use image::{ImageBuffer, ImageFormat, RgbImage};

use crate::helpers::Color;

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    image_data: Vec<u8>,
}

impl Image {
    pub fn new(width: usize, height: usize, image_data: Vec<Color>) -> Self {
        assert!(width * height == image_data.len());
        let image_data = Self::tone_map(width, height, image_data);
        Self {
            width,
            height,
            image_data,
        }
    }

    pub fn save(self, path: &str) -> std::io::Result<()> {
        let image: RgbImage =
            ImageBuffer::from_raw(self.width as u32, self.height as u32, self.image_data)
                .expect("Error creating the image buffer");

        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);
        let image_format = Self::format(path).expect("Invalid image format");

        image.write_to(w, image_format).expect("Error saving image");
        Ok(())
    }

    pub fn valid_format(path: &str) -> bool {
        Self::format(path).is_some()
    }

    fn format(path: &str) -> Option<ImageFormat> {
        let path = Path::new(path);
        match path.extension()?.to_str()? {
            "png" => Some(ImageFormat::Png),
            "jpeg" => Some(ImageFormat::Jpeg),
            "jpg" => Some(ImageFormat::Jpeg),
            "gif" => Some(ImageFormat::Gif),
            "webp" => Some(ImageFormat::WebP),
            "tiff" => Some(ImageFormat::Tiff),
            "tga" => Some(ImageFormat::Tga),
            "bmp" => Some(ImageFormat::Bmp),
            "ico" => Some(ImageFormat::Ico),
            "hdr" => Some(ImageFormat::Hdr),
            "openexr" => Some(ImageFormat::OpenExr),
            "pnm" => Some(ImageFormat::Pnm),
            "farbfeld" => Some(ImageFormat::Farbfeld),
            "avif" => Some(ImageFormat::Avif),
            _ => None,
        }
    }

    fn tone_map(width: usize, height: usize, image_data: Vec<Color>) -> Vec<u8> {
        let mut pixel_data = Vec::with_capacity(width * height);

        for j in 0..height {
            for i in 0..width {
                let index = (j * width) + i;
                let pixel = image_data[index];
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
}
