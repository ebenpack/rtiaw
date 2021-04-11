use crate::color::Color;
use crate::image::{Image, PPM};
use crate::ray::Ray;
use crate::vec3::Vec3;

mod color;
mod image;
mod ray;
mod vec3;

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center.clone();
    let a = r.direction.len_squared();
    let half_b = Vec3::dot(&oc, &r.direction);
    let c = oc.len_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = Vec3::unit_vector(&(ray.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * &Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = Vec3::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * &Color::new(1.0, 1.0, 1.0) + t * &Color::new(0.5, 0.7, 1.0);
}

fn main() -> std::io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    let mut image_data = vec![];

    for j in (0..image_height).rev() {
        if j % (image_height / 10) == 0 && j != 0 {
            println!("{}% completed", (((image_height - j) as f64 / image_height as f64) * 100.0).floor());
        }
        let mut row = vec![];
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * &horizontal + v * &vertical - origin,
            );
            let pixel_color = ray_color(&r);
            row.push(pixel_color)
        }
        image_data.push(row);
    }

    let ppm_image = PPM {
        image_width,
        image_height,
        image_data,
    };
    ppm_image.render_to_file("test.ppm")?;

    Ok(())
}
