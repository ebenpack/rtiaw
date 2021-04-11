use crate::color::Color;
use crate::material::Material;
use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&r_in.direction), &hit_record.normal);
        let new_scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * &Vec3::random_in_unit_sphere(),
        );
        scattered.direction = new_scattered.direction;
        scattered.origin = new_scattered.origin;
        attenuation.red = self.albedo.red;
        attenuation.green = self.albedo.green;
        attenuation.blue = self.albedo.blue;
        Vec3::dot(&scattered.direction, &hit_record.normal) > 0.0
    }
}
