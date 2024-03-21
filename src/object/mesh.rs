use super::intersection::{get_min_intersection, Intersectable, MaterialInformation};
use nalgebra::Vector3;
use tobj::Model;

use super::{bounding_box::BoundingBox, face::Face, intersection::Intersection, ray::Ray};
use crate::helpers::Comparable;

#[derive(Debug, Default)]
pub struct Mesh {
    material_id: Option<usize>,
    name: String,
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
        let (mut min_vert, mut max_vert) = self.faces[0].get_bounding_box().get_min_max();

        for face in self.faces.iter().skip(1) {
            let (face_min_vert, face_max_vert) = face.get_bounding_box().get_min_max();
            min_vert = min_vert.min_between(&face_min_vert);
            max_vert = max_vert.max_between(&face_max_vert);
        }

        self.bounding_box = BoundingBox::new(min_vert, max_vert);
    }

    fn material_id(&self) -> Option<usize> {
        self.material_id
    }
}

impl From<Model> for Mesh {
    fn from(model: Model) -> Self {
        let indices = &model.mesh.indices;
        let mesh = &model.mesh;

        let mut obj = Self {
            name: model.name.clone(),
            material_id: mesh.material_id,
            ..Default::default()
        };

        obj.faces.reserve(mesh.indices.len() / 3);
        let mut next_face = 0;
        for (i, face) in (0..mesh.indices.len() / 3).enumerate() {
            let end = next_face + 3;
            let face_indices = &indices[next_face..end];

            let vertices = [
                Vector3::new(
                    mesh.positions[(face_indices[0] * 3) as usize],
                    mesh.positions[(face_indices[0] * 3 + 1) as usize],
                    mesh.positions[(face_indices[0] * 3 + 2) as usize],
                ),
                Vector3::new(
                    mesh.positions[(face_indices[1] * 3) as usize],
                    mesh.positions[(face_indices[1] * 3 + 1) as usize],
                    mesh.positions[(face_indices[1] * 3 + 2) as usize],
                ),
                Vector3::new(
                    mesh.positions[(face_indices[2] * 3) as usize],
                    mesh.positions[(face_indices[2] * 3 + 1) as usize],
                    mesh.positions[(face_indices[2] * 3 + 2) as usize],
                ),
            ];

            let mut normals = None;
            if !mesh.normals.is_empty() {
                normals = Some([
                    Vector3::new(
                        mesh.normals[(face_indices[0] * 3) as usize],
                        mesh.normals[(face_indices[0] * 3 + 1) as usize],
                        mesh.normals[(face_indices[0] * 3 + 2) as usize],
                    ),
                    Vector3::new(
                        mesh.normals[(face_indices[1] * 3) as usize],
                        mesh.normals[(face_indices[1] * 3 + 1) as usize],
                        mesh.normals[(face_indices[1] * 3 + 2) as usize],
                    ),
                    Vector3::new(
                        mesh.normals[(face_indices[2] * 3) as usize],
                        mesh.normals[(face_indices[2] * 3 + 1) as usize],
                        mesh.normals[(face_indices[2] * 3 + 2) as usize],
                    ),
                ]);
            }

            let texcoords = None;
            // if !mesh.texcoords.is_empty() {
            // texcoords = Some([
            // Vector2::new(
            // mesh.texcoords[(face_indices[0] * 2) as usize],
            // mesh.texcoords[(face_indices[0] * 2 + 1) as usize],
            // ),
            // Vector2::new(
            // mesh.texcoords[(face_indices[1] * 2) as usize],
            // mesh.texcoords[(face_indices[1] * 2 + 1) as usize],
            // ),
            // Vector2::new(
            // mesh.texcoords[(face_indices[2] * 2) as usize],
            // mesh.texcoords[(face_indices[2] * 2 + 1) as usize],
            // ),
            // ]);
            // }

            obj.faces.push(Face::new(i, vertices, normals, texcoords));

            next_face = end;
        }

        obj.update_bounding_box();
        obj
    }
}
