use crate::object::{HitRecord, Object};
use crate::ray::Ray;
use std::sync::Arc;

pub struct Scene {
    pub objects: Vec<Arc<Object>>,
}

impl Object for Scene {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in self.objects {
            if obj.hit(ray, t_min, t_max, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        return hit_anything;
    }
}
