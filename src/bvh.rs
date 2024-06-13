use std::rc::Rc;

use crate::{
    helpers::Vec3,
    object::bounding_box::{Bounded, BoundingBox},
};

struct PrimitiveInfo {
    id: usize,
    bounding_box: BoundingBox,
    centroid: Vec3,
}

impl PrimitiveInfo {
    fn new(id: usize, bounding_box: BoundingBox) -> Self {
        let (min, max) = bounding_box.get_min_max();
        Self {
            id,
            bounding_box,
            centroid: 0.5 * min + 0.5 * max,
        }
    }
}

#[derive(Debug, Default)]
struct BVHNode {
    bounds: BoundingBox,
    children: [Option<Box<BVHNode>>; 2],
    split_axis: usize,
    first_prim_offset: usize,
    number_primitives: usize,
}

impl BVHNode {
    pub fn new_leaf(
        bounds: BoundingBox,
        number_primitives: usize,
        first_prim_offset: usize,
    ) -> Self {
        Self {
            bounds,
            number_primitives,
            first_prim_offset,
            ..Default::default()
        }
    }

    pub fn new_interior(axis: usize, left_child: Box<Self>, right_child: Box<Self>) -> Self {
        let bounds = left_child.bounds.union(&right_child.bounds);
        Self {
            children: [Some(left_child), Some(right_child)],
            bounds,
            split_axis: axis,
            number_primitives: 0,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct BVH {
    root: Box<BVHNode>,
}

impl BVH {
    pub fn new(primitives: &Vec<Rc<dyn Bounded>>) -> Self {
        let mut primitive_info: Vec<PrimitiveInfo> = primitives
            .iter()
            .enumerate()
            .map(|(index, primitive)| PrimitiveInfo::new(index, primitive.bounding_box()))
            .collect();
        let total_nodes = 0;
        let mut ordered_prims: Vec<Rc<dyn Bounded>> = Vec::with_capacity(primitives.len());

        let (root, _) = recursive_build(
            primitives,
            &mut primitive_info,
            0,
            primitives.len(),
            &mut ordered_prims,
            250,
        );
        Self { root }
    }
}

fn recursive_build(
    primitives: &Vec<Rc<dyn Bounded>>,
    primitive_info: &mut Vec<PrimitiveInfo>,
    start: usize,
    end: usize,
    ordered_prims: &mut Vec<Rc<dyn Bounded>>,
    max_prims_in_node: usize,
) -> (Box<BVHNode>, i32) {
    let bounds = primitive_info[start..end]
        .iter()
        .fold(BoundingBox::default(), |acc, next| {
            acc.union(&next.bounding_box)
        });

    let number_primitives = end - start;
    if number_primitives == 1 {
        (
            create_leaf_node(
                primitives,
                primitive_info,
                ordered_prims,
                bounds,
                number_primitives,
            ),
            1,
        )
    } else {
        let centroid_bounds = primitive_info[start..end]
            .iter()
            .fold(BoundingBox::default(), |acc, next| {
                acc.union_with_point(&next.centroid)
            });
        let dim = centroid_bounds.maximum_extent();
        let (min, max) = centroid_bounds.get_min_max();
        if max[dim] == min[dim] {
            (
                create_leaf_node(
                    primitives,
                    primitive_info,
                    ordered_prims,
                    bounds,
                    number_primitives,
                ),
                1,
            )
        } else {
            let mid = (start + end) / 2;
            dbg!(start, end, mid, primitive_info.len());
            primitive_info[start..end].select_nth_unstable_by(mid - start, |a, b| {
                a.centroid[dim]
                    .partial_cmp(&b.centroid[dim])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let (right_child, left_num) = recursive_build(
                primitives,
                primitive_info,
                start,
                mid,
                ordered_prims,
                max_prims_in_node,
            );

            let (left_child, right_num) = recursive_build(
                primitives,
                primitive_info,
                mid,
                end,
                ordered_prims,
                max_prims_in_node,
            );

            (
                Box::new(BVHNode::new_interior(dim, left_child, right_child)),
                1 + left_num + right_num,
            )
        }
    }
}

fn create_leaf_node(
    primitives: &Vec<Rc<dyn Bounded>>,
    primitive_info: &mut Vec<PrimitiveInfo>,
    ordered_prims: &mut Vec<Rc<dyn Bounded>>,
    bounds: BoundingBox,
    number_primitives: usize,
) -> Box<BVHNode> {
    let first_prim_offset = ordered_prims.len();
    for prim in primitive_info {
        let prim_number = prim.id;
        ordered_prims.push(Rc::clone(&primitives[prim_number]));
    }
    Box::new(BVHNode::new_leaf(
        bounds,
        number_primitives,
        first_prim_offset,
    ))
}
