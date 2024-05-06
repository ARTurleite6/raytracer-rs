use crate::helpers::Vec3;
use std::default::Default;

const ADJUST_VALUE: f64 = 0.0001;

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    x: usize,
    y: usize,
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            ..Default::default()
        }
    }

    pub fn new_with_coords(origin: Vec3, direction: Vec3, x: usize, y: usize) -> Self {
        Self {
            origin,
            direction,
            x,
            y,
        }
    }

    pub fn new_with_adjusted_origin(origin: Vec3, direction: Vec3, normal: Vec3) -> Self {
        let mut ray = Self {
            origin,
            direction,
            ..Default::default()
        };

        ray.adjust_origin(normal);
        ray
    }

    pub fn adjust_origin(&mut self, normal: Vec3) {
        let mut offset = ADJUST_VALUE * normal;

        if self.direction.dot(&normal) < 0.0 {
            offset = 1.0 * offset;
        }

        self.origin += offset;
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}
