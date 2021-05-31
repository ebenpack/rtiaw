use crate::aabb::AABB;
use crate::object::{HitRecord, Object, ObjectList};
use crate::ray::Ray;
use rand::Rng;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Object + Send + Sync>,
    right: Arc<dyn Object + Send + Sync>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn new(
        objects: &mut ObjectList,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BVHNode {
        let axis: usize = rand::thread_rng().gen_range(0..=2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (
                objects.objects[start].clone(),
                objects.objects[start].clone(),
            )
        } else if object_span == 2 {
            if comparator(&objects.objects[start], &objects.objects[start + 1])
                == std::cmp::Ordering::Less
            {
                (
                    objects.objects[start].clone(),
                    objects.objects[start + 1].clone(),
                )
            } else {
                (
                    objects.objects[start + 1].clone(),
                    objects.objects[start].clone(),
                )
            }
        } else {
            objects.objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            let a: Arc<dyn Object + Send + Sync> =
                Arc::new(BVHNode::new(objects, start, mid, time0, time1));
            let b: Arc<dyn Object + Send + Sync> =
                Arc::new(BVHNode::new(objects, mid, end, time0, time1));
            (a, b)
        };
        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        match (box_left, box_right) {
            (Some(box_left), Some(box_right)) => {
                let bounding_box = AABB::bounding_box(&box_left, &box_right);
                BVHNode {
                    left,
                    right,
                    bounding_box,
                }
            }
            (_, _) => panic!("No bounding box in bvh_node constructor.\n"),
        }
    }
}

impl Object for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(ray, t_min, if hit_left { rec.t } else { t_max }, rec);

        return hit_left || hit_right;
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

fn box_compare<'r, 's>(
    a: &'r Arc<dyn Object + Send + Sync>,
    b: &'s Arc<dyn Object + Send + Sync>,
    axis: usize,
) -> std::cmp::Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);
    match (box_a, box_b) {
        (Some(box_a), Some(box_b)) => box_a.minimum[axis]
            .partial_cmp(&box_b.minimum[axis])
            .unwrap(),
        (_, _) => panic!("No bounding box in bvh_node constructor.\n"),
    }
}

fn box_x_compare<'r, 's>(
    a: &'r Arc<dyn Object + Send + Sync>,
    b: &'s Arc<dyn Object + Send + Sync>,
) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare<'r, 's>(
    a: &'r Arc<dyn Object + Send + Sync>,
    b: &'s Arc<dyn Object + Send + Sync>,
) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}
fn box_z_compare<'r, 's>(
    a: &'r Arc<dyn Object + Send + Sync>,
    b: &'s Arc<dyn Object + Send + Sync>,
) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}
