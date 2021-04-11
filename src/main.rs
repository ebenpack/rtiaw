use crate::color::Color;
use crate::image::{Image, PPM};
use crate::ray::Ray;
use crate::vec3::Vec3;

mod color;
mod image;
mod ray;
mod vec3;

pub fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t)
        * &Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
        + t * &Color {
            red: 0.5,
            green: 0.7,
            blue: 1.0,
        };
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

    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

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
