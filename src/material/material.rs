use crate::color::Color;
use crate::object::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
