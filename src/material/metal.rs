use crate::color::Color;
use crate::material::Material;
use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
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
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        let new_scattered = Ray::new(rec.p, reflected);
        scattered.direction = new_scattered.direction;
        scattered.origin = new_scattered.origin;
        attenuation.red = albedo.red;
        attenuation.green = albedo.green;
        attenuation.blue = albedo.blue;
        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}
