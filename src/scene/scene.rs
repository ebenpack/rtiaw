use crate::aabb::AABB;
use crate::object::{BVHNode, HitRecord, Object, ObjectList};
use crate::ray::Ray;

pub struct Scene {
    pub objects: BVHNode,
}

impl Scene {
    pub fn new(src_objects: &mut ObjectList) -> Scene {
        Scene {
            objects: BVHNode::new(src_objects, 0, src_objects.objects.len(), 0.0, 0.0),
        }
    }
}

impl Object for Scene {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.objects.hit(ray, t_min, t_max, rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.objects.bounding_box(time0, time1)
        // if self.objects.is_empty() {
        //     return None;
        // };
        //
        // let temp_box = AABB::new(Vec3::origin(), Vec3::origin());
        // let mut first_box = true;
        // let mut output_box = AABB::new(Vec3::origin(), Vec3::origin());
        //
        // for object in self.objects.iter() {
        //     match object.bounding_box(time0, time1) {
        //         None => return None,
        //         Some(temp_box) => {
        //             output_box = if first_box {
        //                 temp_box
        //             } else {
        //                 AABB::surrounding_box(&output_box, &temp_box)
        //             };
        //             first_box = false;
        //         }
        //     }
        // }
        //
        // Some(output_box)
    }
}
