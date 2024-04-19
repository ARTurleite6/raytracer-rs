use crate::helpers::Vec3;

use super::ray::Ray;

#[derive(Debug, Default)]
pub struct BoundingBox {
    min: Vec3,
    max: Vec3,
}

impl BoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn get_min_max(&self) -> (Vec3, Vec3) {
        (self.min, self.max)
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
}
