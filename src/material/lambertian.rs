use crate::color::Color;
use crate::material::Material;
use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let new_scattered = Ray::new(hit_record.p.clone(), scatter_direction);

        // TODO: More succinct way of doing this?
        scattered.origin = new_scattered.origin;
        scattered.direction = new_scattered.direction;

        let new_attenuation = self.albedo;
        attenuation.red = new_attenuation.red;
        attenuation.green = new_attenuation.green;
        attenuation.blue = new_attenuation.blue;
        return true;
    }
}
