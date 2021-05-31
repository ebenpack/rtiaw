use crate::aabb::AABB;
use crate::object::{HitRecord, Object};
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct ObjectList {
    pub objects: Vec<Arc<dyn Object + Send + Sync>>,
}

impl ObjectList {
    pub fn new(objects: Vec<Arc<dyn Object + Send + Sync>>) -> Self {
        ObjectList { objects }
    }
}

impl Object for ObjectList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if obj.hit(ray, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        };

        let mut first_box = true;
        let mut output_box = AABB::new(Vec3::origin(), Vec3::origin());

        for object in self.objects.iter() {
            match object.bounding_box(time0, time1) {
                None => return None,
                Some(temp_box) => {
                    output_box = if first_box {
                        first_box = false;
                        temp_box
                    } else {
                        AABB::bounding_box(&output_box, &temp_box)
                    };
                }
            }
        }

        Some(output_box)
    }
}
