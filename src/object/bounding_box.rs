use nalgebra::Vector3;

use super::ray::Ray;

#[derive(Debug, Default)]
pub struct BoundingBox {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl BoundingBox {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min, max }
    }

    pub fn get_min_max(&self) -> (Vector3<f32>, Vector3<f32>) {
        (self.min, self.max)
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let origin = ray.get_origin();
        let direction = ray.get_direction();

        let t_x_min = (self.min.x - origin.x) / direction.x;
        let t_x_max = (self.max.x - origin.x) / direction.x;

        let t_min = t_x_min.min(t_x_max);
        let t_max = t_x_min.max(t_x_max);

        let t_y_min = (self.min.y - origin.y) / direction.y;
        let t_y_max = (self.max.y - origin.y) / direction.y;

        let t_min = t_y_min.min(t_y_max);
        let t_max = t_y_min.max(t_y_max);

        let t_z_min = (self.min.z - origin.z) / direction.z;
        let t_z_max = (self.max.z - origin.z) / direction.z;

        let t_min = t_z_min.min(t_z_max);
        let t_max = t_z_min.max(t_z_max);

        t_max >= t_min
    }
}
