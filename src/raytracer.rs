use crate::camera::Camera;
use crate::image::Image;

use crate::material::Material;
use crate::material::dielectric::Dielectric;
use crate::material::metal::Metal;
use crate::material::lambertian::Lambertian;


use crate::object::sphere::Sphere;

use crate::object::{Hit, Object};
use crate::ray::Ray;
use crate::vec3::Vec3;

use rand::Rng;
use std::f32;
use indicatif::ProgressBar;
use rayon::prelude::*;

const SAMPLES: i32 = 32;
pub const WIDTH: usize = 1920;
pub const HEIGHT: usize = 1080;

fn build_scene() -> Vec<Box<Object>> {
    vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Box::new(Dielectric::new(1.5)),
        )),
    ]
}
fn build_random_scene() -> Vec<Box<Object>> {
    let rand = || {rand::thread_rng().gen::<f32>()};
    let mut list: Vec<Box<Object>> = (-11..11).flat_map(|a| {
        (-11..11).map(move |b| {
            let choose = rand();
            let center = Vec3::new((a as f32)+0.9*rand(), 0.2, (b as f32) + 0.9*rand());
            if (center - Vec3::new(4.0,0.2,0.0)).len() > 0.9 {
            let mat :Box<Material> = if choose < 0.8 {
                  Box::new(Lambertian::new(Vec3::new(rand()*rand(),rand()*rand(),rand()*rand())))
            }else if choose < 0.95 {
                Box::new(Metal::new(Vec3::new(0.5*(1.0+rand()), 0.5*(1.0+rand()), 0.5*(1.0+rand())), 0.5*rand()))
            }else {
                Box::new(Dielectric::new(1.5))
            };
            let tmp : Box<Object> = Box::new(Sphere::new(center, 0.2, mat));
            Some(tmp)
            } else {
                None
            }
        })
    }).flat_map(|a| a).collect::<Vec<Box<Object>>>();
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0,1.0,0.0), 1.0, Box::new(Dielectric::new(1.5))
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0,1.0,0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4,0.2,0.1)))
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0,1.0,0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7,0.6,0.5), 0.0))
    )));

    list
}

pub fn raytrace(image: &mut Image) {
    let width = image[0].len();
    let height = image.len();
    let aspect = (width as f32) / (height as f32);
    let from = Vec3::new(13.0, 2.0, 3.0);
    //let from = Vec3::new(0.0,0.0,0.0);
    let to = Vec3::new(0.0, 2.0, 0.0);
    let dist_to_focus = (to - from).len();
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    let camera = Camera::new(
        from,
        to,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect,
        aperature,
        dist_to_focus,
    );
    let objs = build_random_scene();
    //let objs = build_scene();
    let objs = &objs;
    println!("Width: {}\nHeight: {}", width, height);
    let bar = ProgressBar::new((height) as u64);
    image.par_iter_mut().enumerate().for_each(|(j,row)| {
        let mut rng = rand::thread_rng();
        for (i, vec) in row.iter_mut().enumerate() {
            *vec = (0..SAMPLES)
                .map(|_| {
                    let u = ((i as f32) + rng.gen::<f32>()) / (width as f32);
                    let v = (((height - j) as f32) + rng.gen::<f32>()) / (height as f32);

                    let ray = camera.ray(u, v);
                    color(&ray, objs, 0)
                })
                .fold(Vec3::from(0.0), |a, b| a + b)
                / (SAMPLES as f32);
        }
                bar.inc(1);
    });
    bar.finish();
    println!("Done!");
}

fn get_hit<'a>(ray: &Ray, objs: &'a Vec<Box<Object>>) -> Option<Hit<'a>> {
    objs.iter()
        .map(|obj| (*obj).hits(ray, 0.01, f32::MAX))
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
