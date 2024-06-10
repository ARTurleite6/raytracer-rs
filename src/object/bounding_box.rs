use serde::Deserialize;

use crate::helpers::Vec3;

use super::ray::Ray;

pub trait Bounded {
    fn bounding_box(&self) -> BoundingBox;
}

#[derive(Debug, Default, Clone, Deserialize, Copy)]
pub struct BoundingBox {
    min: Vec3,
    max: Vec3,
}

impl BoundingBox {
    pub fn new(min: &Vec3, max: &Vec3) -> Self {
        Self {
            min: min.clone(),
            max: max.clone(),
        }
    }

    pub fn get_min_max(&self) -> (&Vec3, &Vec3) {
        (&self.min, &self.max)
    }

    pub fn maximum_extent(&self) -> usize {
        let diagonal = self.diagonal();

        if diagonal.x > diagonal.y && diagonal.x > diagonal.z {
            0
        } else if diagonal.y > diagonal.z {
            1
        } else {
            2
        }
    }

    pub fn union_with_point(&self, other: &Vec3) -> Self {
        Self::new(
            &Vec3::new(
                self.min.x.min(other.x),
                self.min.y.min(other.y),
                self.min.z.min(other.z),
            ),
            &Vec3::new(
                self.max.x.max(other.x),
                self.max.y.max(other.y),
                self.max.z.max(other.z),
            ),
        )
    }

    pub fn union(&self, other: &BoundingBox) -> Self {
        Self::new(
            &Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            &Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        )
    }

    pub fn surface_area(&self) -> f64 {
        let d = self.diagonal();
        2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
    }

    pub fn offset(&self, point: &Vec3) -> Vec3 {
        let mut o = point - self.min;
        if self.max.x > self.min.x {
            o.x /= self.max.x - self.min.x;
        }
        if self.max.y > self.min.y {
            o.y /= self.max.y - self.min.y;
        }
        if self.max.z > self.min.z {
            o.z /= self.max.z - self.min.z;
        }
        o
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let origin = ray.origin();
        let direction = ray.direction();

        let t_x_min = (self.min.x - origin.x) / direction.x;
        let t_x_max = (self.max.x - origin.x) / direction.x;

        let t_min = t_x_min.min(t_x_max);
        let t_max = t_x_min.max(t_x_max);

        let t_y_min = (self.min.y - origin.y) / direction.y;
        let t_y_max = (self.max.y - origin.y) / direction.y;

        let t_min = t_min.max(t_y_min.min(t_y_max));
        let t_max = t_max.min(t_y_min.max(t_y_max));

        let t_z_min = (self.min.z - origin.z) / direction.z;
        let t_z_max = (self.max.z - origin.z) / direction.z;

        let t_min = t_min.max(t_z_min.min(t_z_max));
        let t_max = t_max.min(t_z_min.max(t_z_max));

        t_max >= t_min
    }

    fn diagonal(&self) -> Vec3 {
        self.max - self.min
    }
}
