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
    root: BVHNode,
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
) -> (BVHNode, i32) {
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
        let mut mid = (start + end) / 2;
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
            if number_primitives <= 4 {
                let mid = (start + end) / 2;
                primitive_info[start..end].select_nth_unstable_by(mid, |a, b| {
                    a.centroid[dim]
                        .partial_cmp(&b.centroid[dim])
                        .expect("Error comparing centroids")
                });
            } else {
                const NUMBER_BUCKETS: usize = 12;
                #[derive(Default, Clone, Copy)]
                struct BucketInfo {
                    count: usize,
                    bounds: BoundingBox,
                }

                let mut buckets = [BucketInfo::default(); NUMBER_BUCKETS];

                for i in start..end {
                    let mut b = (NUMBER_BUCKETS as f64
                        * centroid_bounds.offset(&primitive_info[i].centroid)[dim])
                        as usize;
                    if b == NUMBER_BUCKETS {
                        b = NUMBER_BUCKETS - 1;
                    }
                    buckets[b].count += 1;
                    buckets[b].bounds = buckets[b].bounds.union(&primitive_info[i].bounding_box);
                }

                let mut costs = [0.0 as f64; NUMBER_BUCKETS - 1];
                for i in 0..(NUMBER_BUCKETS - 1) {
                    let mut b0 = BoundingBox::default();
                    let mut b1 = BoundingBox::default();
                    let mut count0 = 0;
                    let mut count1 = 0;
                    for j in 0..=i {
                        b0 = b0.union(&buckets[j].bounds);
                        count0 += buckets[j].count;
                    }

                    for j in (i + 1)..NUMBER_BUCKETS {
                        b1 = b1.union(&buckets[j].bounds);
                        count1 += buckets[j].count;
                    }
                    costs[i] = 0.125
                        + (count0 as f64 * b0.surface_area() + count1 as f64 * b1.surface_area())
                            / bounds.surface_area()
                }

                let mut min_cost = costs[0];
                let mut min_cost_split_bucket = 0;
                for i in 1..(NUMBER_BUCKETS - 1) {
                    if costs[i] < min_cost {
                        min_cost = costs[i];
                        min_cost_split_bucket = i;
                    }
                }

                let leaf_cost = number_primitives;
                if number_primitives > max_prims_in_node || min_cost < leaf_cost as f64 {
                    mid = primitive_info[start..end].partition_point(|pi| {
                        let mut b = (NUMBER_BUCKETS as f64
                            * centroid_bounds.offset(&pi.centroid)[dim])
                            as usize;
                        if b == NUMBER_BUCKETS {
                            b = NUMBER_BUCKETS - 1;
                        }
                        b <= min_cost_split_bucket
                    });
                } else {
                    return (
                        create_leaf_node(
                            primitives,
                            primitive_info,
                            ordered_prims,
                            bounds,
                            number_primitives,
                        ),
                        1,
                    );
                }
            }
            let (children_left, left_nodes) = recursive_build(
                primitives,
                primitive_info,
                start,
                mid,
                ordered_prims,
                max_prims_in_node,
            );
            let (children_right, right_nodes) = recursive_build(
                primitives,
                primitive_info,
                mid,
                end,
                ordered_prims,
                max_prims_in_node,
            );
            (
                BVHNode::new_interior(dim, Box::new(children_left), Box::new(children_right)),
                left_nodes + right_nodes,
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
) -> BVHNode {
    let first_prim_offset = ordered_prims.len();
    for prim in primitive_info {
        let prim_number = prim.id;
        ordered_prims.push(Rc::clone(&primitives[prim_number]));
    }
    BVHNode::new_leaf(bounds, number_primitives, first_prim_offset)
}
