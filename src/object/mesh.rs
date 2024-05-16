use super::{
    face::FaceBuilder,
    intersection::{get_min_intersection, Intersectable, MaterialInformation},
};
use nalgebra::Vector3;
use tobj::Model;

use super::{bounding_box::BoundingBox, face::Face, intersection::Intersection, ray::Ray};
use crate::helpers::Comparable;

#[derive(Debug, Default)]
pub struct Mesh {
    material_id: Option<usize>,
    faces: Vec<Face>,
    bounding_box: BoundingBox,
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if !self.bounding_box.intersect(ray) {
            return None;
        }

        let mut intersection = get_min_intersection(ray, &self.faces);
        if let Some(material_id) = self.material_id {
            if let Some(intersection) = &mut intersection {
                intersection.brdf = Some(MaterialInformation {
                    material: None,
                    material_id,
                });
            }
        }

        intersection
    }
}

impl Mesh {
    fn update_bounding_box(&mut self) {
        let (&(mut min_vert), &(mut max_vert)) = self.faces[0].get_bounding_box().get_min_max();

        for face in self.faces.iter().skip(1) {
            let (face_min_vert, face_max_vert) = face.get_bounding_box().get_min_max();
            min_vert = min_vert.min_between(face_min_vert);
            max_vert = max_vert.max_between(face_max_vert);
        }

        self.bounding_box = BoundingBox::new(&min_vert, &max_vert);
    }
}

impl From<Model> for Mesh {
    fn from(model: Model) -> Self {
        let indices = &model.mesh.indices;
        let mesh = &model.mesh;

        let mut obj = Self {
            material_id: mesh.material_id,
            ..Default::default()
        };

        obj.faces.reserve(mesh.indices.len() / 3);
        let mut next_face = 0;
        for face in 0..mesh.indices.len() / 3 {
            let end = next_face + 3;
            let face_indices = &indices[next_face..end];

            let vertices = [
                Vector3::new(
                    mesh.positions[(face_indices[0] * 3) as usize] as f64,
                    mesh.positions[(face_indices[0] * 3 + 1) as usize] as f64,
                    mesh.positions[(face_indices[0] * 3 + 2) as usize] as f64,
                ),
                Vector3::new(
                    mesh.positions[(face_indices[1] * 3) as usize] as f64,
                    mesh.positions[(face_indices[1] * 3 + 1) as usize] as f64,
                    mesh.positions[(face_indices[1] * 3 + 2) as usize] as f64,
                ),
                Vector3::new(
                    mesh.positions[(face_indices[2] * 3) as usize] as f64,
                    mesh.positions[(face_indices[2] * 3 + 1) as usize] as f64,
                    mesh.positions[(face_indices[2] * 3 + 2) as usize] as f64,
                ),
            ];

            obj.faces
                .push(FaceBuilder::new(vertices).face_id(face).build());

            next_face = end;
        }

        obj.update_bounding_box();
        obj
    }
}
