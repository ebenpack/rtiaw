use crate::color::Color;
use crate::material::Material;
use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }
    fn reflectance(cosine: f64, reflectance_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - reflectance_idx) / (1.0 + reflectance_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        attenuation.red = 1.0;
        attenuation.green = 1.0;
        attenuation.blue = 1.0;
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(&r_in.direction);

        let cos_theta = Vec3::dot(&-unit_direction.clone(), &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let rand_num = rand::thread_rng().gen_range(0.0..1.0);
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand_num {
                Vec3::reflect(&unit_direction, &hit_record.normal)
            } else {
                Vec3::refract(&unit_direction, &hit_record.normal, refraction_ratio)
            };

        let new_scattered = Ray::new(hit_record.p.clone(), direction);
        scattered.origin = new_scattered.origin;
        scattered.direction = new_scattered.direction;
        true
    }
}
