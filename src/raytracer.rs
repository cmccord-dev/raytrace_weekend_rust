use crate::Camera;
use crate::Image;

use crate::World;
use crate::Ray;
use crate::Vec3;
use crate::Object;

use rand::Rng;
use std::f32;
use indicatif::ProgressBar;
use rayon::prelude::*;

const SAMPLES: i32 = 10;
pub const WIDTH: usize = 1920;
pub const HEIGHT: usize = 1080;


pub fn raytrace(image: &mut Image) {
    let width = image[0].len();
    let height = image.len();
    let aspect = (width as f32) / (height as f32);
    let from = Vec3::new(13.0, 2.0, 3.0);
    //let from = Vec3::new(0.0,0.0,0.0);
    let to = Vec3::new(0.0, 2.0, 0.0);
    //let dist_to_focus = (to - from).len();
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
    let world = World::build_random_scene();
    //let world = build_scene();
    let world = &world;
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
                    color(&ray, world, 0)
                })
                .fold(Vec3::from(0.0), |a, b| a + b)
                / (SAMPLES as f32);
        }
                bar.inc(1);
    });
    bar.finish();
    println!("Done!");
}


fn color(ray: &Ray, world: &World, depth: i32) -> Vec3 {
    if let Some(tmp) = world.hits(ray, 0.01, f32::MAX) {
        if depth < 50 {
            match tmp.material.scatter(ray, &tmp) {
                Some(scatter) => scatter.attenuation * color(&scatter.scattered, world, depth + 1),
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
