use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use png::Encoder;

pub struct Image {
    width: u32,
    height: u32,
    image_data: Vec<f32>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            image_data: Vec::new(),
        }
    }

    pub fn write_png(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(filename);
        let file = File::create(path)?;
        let ref mut writer = BufWriter::new(file);

        let mut encoder = Encoder::new(writer, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut png_writer = encoder.write_header()?;
        let mut png_data = Vec::new();

        for pixel in &self.image_data {
            let r = (pixel * 255.0) as u8;
            let g = (pixel * 255.0) as u8;
            let b = (pixel * 255.0) as u8;
            let a = 255;

            png_data.push(r);
            png_data.push(g);
            png_data.push(b);
            png_data.push(a);
        }

        png_writer.write_image_data(&png_data)?;

        Ok(())
    }
}
