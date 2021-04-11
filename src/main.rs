use crate::color::Color;
use crate::image::{Image, PPM};
use crate::object::{HitRecord, Object, Sphere};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::Vec3;
use std::sync::Arc;
use std::thread;

mod color;
mod image;
mod object;
mod ray;
mod scene;
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

fn ray_color(ray: &Ray, scene: &Scene) -> Color {
    let mut rec = HitRecord {
        p: Vec3::origin(),
        normal: Vec3::origin(),
        t: 0.0,
        front_face: false,
    };
    if scene.hit(ray, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * &(rec.normal + Color::new(1.0, 1.0, 1.0));
    }
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
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Arc::new(Vec3::new(0.0, 0.0, 0.0));
    let horizontal = Arc::new(Vec3::new(viewport_width, 0.0, 0.0));
    let vertical = Arc::new(Vec3::new(0.0, viewport_height, 0.0));
    let lower_left_corner = Arc::new(
        *origin - (*horizontal / 2.0) - (*vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length),
    );

    let mut scene = Scene::new();
    scene.add(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    scene.add(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let scene = Arc::new(scene);

    struct Job {
        x: i32,
        y: i32,
    }

    struct Result {
        x: i32,
        y: i32,
        color: Color,
    }

    let (job_sender, job_receiver) = crossbeam::channel::unbounded::<Job>();
    let (result_sender, result_receiver) = crossbeam::channel::unbounded::<Result>();

    let mut thread_handles = vec![];
    for _ in 0..num_cpus::get() {
        let job_receiver = job_receiver.clone();
        let result_sender = result_sender.clone();
        let scene = Arc::clone(&scene);
        let origin = Arc::clone(&origin);
        let horizontal = Arc::clone(&horizontal);
        let vertical = Arc::clone(&vertical);
        let lower_left_corner = Arc::clone(&lower_left_corner);

        thread_handles.push(thread::spawn(move || {
            while let Ok(next_job) = job_receiver.recv() {
                let u = next_job.x as f64 / (image_width - 1) as f64;
                let v = next_job.y as f64 / (image_height - 1) as f64;
                let ray = Ray::new(
                    *origin,
                    (*lower_left_corner) + (u * &(*horizontal)) + v * &(*vertical) - (*origin),
                );
                let color = ray_color(&ray, &scene);
                result_sender
                    .send(Result {
                        x: next_job.x,
                        y: next_job.y,
                        color,
                    })
                    .expect("Tried writing to channel, but there are no receivers!");
            }
        }));
    }
    drop(result_sender);

    for y in 0..image_height {
        for x in 0..image_width {
            job_sender
                .send(Job { x, y })
                .expect("Tried writing to channel, but there are no receivers!");
        }
    }
    drop(job_sender);

    for handle in thread_handles {
        handle.join().expect("Panic occurred in thread")
    }

    let mut image_data = Vec::with_capacity((image_width * image_height) as usize);

    // TODO: Improve this? fill, or something?
    for _ in 0..(image_width * image_height) {
        image_data.push(Color::new(0.0, 0.0, 0.0));
    }

    while let Ok(next_result) = result_receiver.recv() {
        // Flip you. Flip you for real.
        let y = image_height - 1 - next_result.y;
        let index = (y * image_width + next_result.x) as usize;
        image_data[index] = next_result.color;
    }

    let ppm_image = PPM {
        image_width,
        image_height,
        image_data,
    };
    ppm_image.render_to_file("test.ppm")?;

    Ok(())
}
