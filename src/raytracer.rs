use crate::camera::Camera;
use crate::image::Image;

use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::dielectric::Dielectric;
use crate::object::sphere::Sphere;

use crate::object::{Hit, Object};
use crate::ray::Ray;
use crate::vec3::Vec3;

use rand::Rng;
use std::f32;
use std::rc::Rc;

const SAMPLES: i32 = 50;

fn build_scene() -> Vec<Box<Object>> {
    vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Rc::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Rc::new(Dielectric::new(1.5)),
        )),
    ]
}

pub fn raytrace(image: &mut Image) {
    let width = image[0].len();
    let height = image.len();
    let aspect = (height as f32) / (width as f32);
    let camera = Camera::new(
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0 * aspect, 0.0),
        Vec3::from_value(0.0),
    );
    let mut rng = rand::thread_rng();
    let objs = build_scene();

    println!("Width: {}\nHeight: {}", width, height);
    for (j, row) in image.iter_mut().enumerate() {
        for (i, vec) in row.iter_mut().enumerate() {
            *vec = (0..SAMPLES)
                .map(|_| {
                    let u = ((i as f32) + rng.gen::<f32>()) / (width as f32);
                    let v = (((height - j) as f32) + rng.gen::<f32>()) / (height as f32);

                    let ray = camera.ray(u, v);
                    color(&ray, &objs, 0)
                })
                .fold(Vec3::from(0.0), |a, b| a + b)
                / (SAMPLES as f32);
        }
    }
    println!("Done!");
}

fn get_hit(ray: &Ray, objs: &Vec<Box<Object>>) -> Option<Hit> {
    objs.iter()
        .map(|obj| (*obj).hits(ray, 0.001, f32::MAX))
        .filter_map(|hit| hit)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal))
}

fn color(ray: &Ray, objs: &Vec<Box<Object>>, depth: i32) -> Vec3 {
    if let Some(tmp) = get_hit(ray, objs) {
        if depth < 50 {
            match tmp.material.scatter(ray, &tmp) {
                Some(scatter) => scatter.attenuation * color(&scatter.scattered, objs, depth + 1),
                None => Vec3::from(0.0),
            }
        } else {
            Vec3::from(0.0)
        }
    } else {
        let t: f32 = 0.5 * (ray.dir.y + 1.0);
        Vec3::lerp(&Vec3::from_value(1.0), &Vec3::new(0.5, 0.7, 1.0), t)
    }
}
