use nalgebra::Vector3;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    image_data: Vec<Vector3<f32>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Result<Self, Box<dyn Error>> {
        if width == 0 || height == 0 {
            return Err("Invalid image size".into());
        }

        dbg!(width * height);
        let image_data = vec![Vector3::<f32>::default(); (width * height) as usize];
        dbg!(&image_data.len());
        Ok(Self {
            width,
            height,
            image_data,
        })
    }

    pub fn set_pixel(
        &mut self,
        x: usize,
        y: usize,
        color: Vector3<f32>,
    ) -> Result<(), Box<dyn Error>> {
        if x >= self.width || y >= self.height {
            return Err("Pixel out of bounds".into());
        }
        self.image_data[(y * self.width + x) as usize] = color;
        Ok(())
    }

    pub fn add_pixel(
        &mut self,
        x: usize,
        y: usize,
        color: Vector3<f32>,
    ) -> Result<(), Box<dyn Error>> {
        //TODO: Perguntar ao stor possivel erro
        if x >= self.width || y >= self.height {
            return Err("Pixel out of bounds".into());
        }
        self.image_data[(y * self.width + x) as usize] += color;
        Ok(())
    }

    fn tone_map(&self) -> Vec<Vector3<u8>> {
        let mut pixel_data = vec![Vector3::<u8>::default(); (self.width * self.height) as usize];

        for j in 0..self.height {
            for i in 0..self.width {
                let index = ((j * self.width) + i) as usize;
                let pixel = self.image_data[index];
                let pixel_color = Vector3::<u8>::new(
                    (pixel.x.min(1.0) * 255.0) as u8,
                    (pixel.y.min(1.0) * 255.0) as u8,
                    (pixel.z.min(1.0) * 255.0) as u8,
                );
                pixel_data[index] = pixel_color;
            }
        }
        pixel_data
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let data = self.tone_map();

        let mut file = File::create(path)?;

        let _ = file.write(std::format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes())?;

        for pixel in data {
            let _ = file.write(&[pixel.x, pixel.y, pixel.z])?;
        }
        Ok(())
    }
}
