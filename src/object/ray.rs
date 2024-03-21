use crate::helpers::Vec3;

#[derive(Debug)]
pub struct Ray {
    x: usize,
    y: usize,
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, x: usize, y: usize) -> Self {
        Self {
            origin,
            direction,
            x,
            y,
        }
    }

    pub fn get_origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
