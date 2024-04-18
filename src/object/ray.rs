use crate::helpers::Vec3;
use std::{default::Default, f32::EPSILON};

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

    pub fn adjust_origin(&mut self, normal: Vec3) {
        let mut offset = EPSILON * normal;

        if self.direction.dot(&normal) < 0.0 {
            offset.neg_mut();
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
