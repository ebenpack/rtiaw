use crate::camera::Camera;
use crate::color::Color;
use crate::image::{Image, PPM};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::object::{HitRecord, Object, Sphere};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::Vec3;
use clap::App;
use rand::Rng;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Result as IoResult};
use std::sync::Arc;
use std::thread;

#[macro_use]
extern crate clap;

mod camera;
mod color;
mod image;
mod material;
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

fn ray_color(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    let mut hit_record = HitRecord {
        p: Vec3::origin(),
        normal: Vec3::origin(),
        t: 0.0,
        front_face: false,
        material: Arc::new(Metal::new(Color::new(0.0, 0.0, 0.0), 1.0)),
    };
    let mut ray = ray.clone();
    let mut depth = depth;
    let mut color = Color::new(1.0, 1.0, 1.0);

    loop {
        if depth <= 0 {
            color *= Color::new(0.0, 0.0, 0.0);
            return color;
        }
        if scene.hit(&ray, 0.001, f64::INFINITY, &mut hit_record) {
            let mut scattered = Ray::new(Vec3::origin(), Vec3::origin());
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            if hit_record
                .material
                .scatter(&ray, &hit_record, &mut attenuation, &mut scattered)
            {
                color *= attenuation;
                ray = scattered.clone();
                depth -= 1;
                continue;
            }
            color += Color::new(0.0, 0.0, 0.0);
            return color;
        }
        let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);
        if t > 0.0 {
            let n = Vec3::unit_vector(&(ray.at(t) - Vec3::new(0.0, 0.0, -1.0)));

            color *= 0.5 * &Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
            return color;
        }
        let unit_direction = Vec3::unit_vector(&ray.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        color *= (1.0 - t) * &Color::new(1.0, 1.0, 1.0) + t * &Color::new(0.5, 0.7, 1.0);
        return color;
    }
}

fn random_scene() -> Scene {
    let mut scene = Scene::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    scene.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::thread_rng().gen_range(0.0..1.0);
            let rand_num1 = rand::thread_rng().gen_range(0.0..1.0);
            let rand_num2 = rand::thread_rng().gen_range(0.0..1.0);

            let center = Vec3::new(a as f64 + 0.9 * rand_num1, 0.2, b as f64 + 0.9 * rand_num2);

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    scene.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    scene.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    scene.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    scene.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    scene.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    scene.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    scene
}

fn main() -> IoResult<()> {
    // Image
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    // TODO: Exit on ill-typed args
    let aspect_ratio = matches
        .value_of_t::<f64>("aspect-ratio")
        .unwrap_or(3.0 / 2.0);
    let image_width = matches.value_of_t::<u32>("image-width").unwrap_or(300);
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = matches.value_of_t::<u32>("samples").unwrap_or(500);
    let max_depth = matches.value_of_t::<u32>("depth").unwrap_or(200).max(500);
    let output_file = matches
        .value_of_os("OUTPUT")
        .map(|f| f.to_str())
        .flatten()
        .ok_or(IoError::new(
            IoErrorKind::InvalidInput,
            "Must provide output file".to_string(),
        ))?;

    // Camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Arc::new(Camera::new(
        &look_from,
        &look_at,
        &vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    ));

    // Scene
    let scene = Arc::new(random_scene());

    struct Job {
        x: u32,
        y: u32,
    }

    struct Result {
        x: u32,
        y: u32,
        color: Color,
    }

    let (job_sender, job_receiver) = crossbeam::channel::unbounded::<Job>();
    let (result_sender, result_receiver) = crossbeam::channel::unbounded::<Result>();

    let mut thread_handles = vec![];
    for _ in 0..num_cpus::get() {
        let job_receiver = job_receiver.clone();
        let result_sender = result_sender.clone();
        let scene = Arc::clone(&scene);
        let camera = Arc::clone(&camera);

        thread_handles.push(thread::spawn(move || {
            while let Ok(next_job) = job_receiver.recv() {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..samples_per_pixel {
                    let rand_num1 = rand::thread_rng().gen_range(0.0..1.0);
                    let rand_num2 = rand::thread_rng().gen_range(0.0..1.0);
                    let u = (next_job.x as f64 + rand_num1) / (image_width as f64 - 1.0);
                    let v = (next_job.y as f64 + rand_num2) / (image_height as f64 - 1.0);
                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(&ray, &scene, max_depth);
                }

                // TODO: Fix this bit up
                let scale = 1.0 / samples_per_pixel as f64;
                pixel_color.red = (scale * pixel_color.red).sqrt().clamp(0.0, 0.9999999);
                pixel_color.green = (scale * pixel_color.green).sqrt().clamp(0.0, 0.9999999);
                pixel_color.blue = (scale * pixel_color.blue).sqrt().clamp(0.0, 0.9999999);

                result_sender
                    .send(Result {
                        x: next_job.x,
                        y: next_job.y,
                        color: pixel_color,
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

    let image_data_size = (image_width * image_height) as usize;
    let mut image_data = Vec::<Color>::with_capacity(image_data_size);

    image_data.resize_with(image_data_size, Default::default);

    let mut processed = 0;
    let total = image_width * image_height;
    let percent = (total as f64 / 100.0).ceil() as i32;
    while let Ok(next_result) = result_receiver.recv() {
        processed += 1;
        if processed % (percent * 5) == 0 {
            let percentage = (100.0 * (processed as f64 / total as f64)).floor() as i32;
            println!("{}% completed", percentage);
        }
        // Flip you. Flip you for real.
        let y = image_height - 1 - next_result.y;
        let index = (y * image_width + next_result.x) as usize;
        // TODO: stream data to the file... somehow
        // stuff's gonna be out of order though, so this
        // will likely be a bit messy
        image_data[index] = next_result.color;
    }

    for handle in thread_handles {
        handle.join().expect("Panic occurred in thread")
    }

    let ppm_image = PPM {
        image_width,
        image_height,
        image_data,
    };
    ppm_image.render_to_file(output_file)?;

    Ok(())
}
