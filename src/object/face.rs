use nalgebra::{Vector2, Vector3};

use super::{
    bounding_box::BoundingBox,
    intersection::{Intersectable, Intersection},
    ray::Ray,
};

#[derive(Debug)]
pub struct Face {
    face_id: usize,
    vertex: [Vector3<f32>; 3],
    normal_coordinates: Option<[Vector3<f32>; 3]>,
    texture_coordinates: Option<[Vector2<f32>; 3]>,
    bounding_box: BoundingBox,
}

impl Face {
    pub fn new(
        face_id: usize,
        vertex: [Vector3<f32>; 3],
        normal_coordinates: Option<[Vector3<f32>; 3]>,
        texture_coordinates: Option<[Vector2<f32>; 3]>,
    ) -> Self {
        let bounding_box = BoundingBox::new(
            vertex.iter().fold(vertex[0], |acc, new_vertex| {
                Vector3::new(
                    acc.x.min(new_vertex.x),
                    acc.y.min(new_vertex.y),
                    acc.z.min(new_vertex.z),
                )
            }),
            vertex.iter().fold(vertex[0], |acc, new_vertex| {
                Vector3::new(
                    acc.x.max(new_vertex.x),
                    acc.y.max(new_vertex.y),
                    acc.z.max(new_vertex.z),
                )
            }),
        );
        Self {
            face_id,
            vertex,
            texture_coordinates,
            normal_coordinates,
            bounding_box,
        }
    }

    pub fn get_bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
}

impl Intersectable for Face {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if !self.bounding_box.intersect(ray) {
            return None;
        }
        let edge_1 = self.vertex[1] - self.vertex[0];
        let edge_2 = self.vertex[2] - self.vertex[0];
        let plane_normal = edge_1.cross(&edge_2);
        let det = -ray.get_direction().dot(&plane_normal);
        let inv_det = 1.0 / det;
        let ao = ray.get_origin() - self.vertex[0];
        let dao = ao.cross(ray.get_direction());
        let u = edge_2.dot(&dao) * inv_det;
        let v = -edge_1.dot(&dao) * inv_det;
        let t = ao.dot(&plane_normal) * inv_det;

        if t >= 0.0 && u > 0.0 && v > 0.0 && u + v <= 1.0 {
            let point = ray.get_origin() + t * ray.get_direction();
            return Some(Intersection::new(
                point,
                plane_normal,
                plane_normal,
                -ray.get_direction(),
                t,
                self.face_id,
            ));
        }
        None
    }
}
