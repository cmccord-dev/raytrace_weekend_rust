
use crate::World;
use crate::Ray;
use crate::Vec3;
use crate::Object;

use rand::Rng;
use std::f32;
use indicatif::ProgressBar;
use rayon::prelude::*;

const SAMPLES: i32 = 1;
pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;


pub fn raytrace(image: &mut Vec<Vec<Vec3>>) {
    let width = image[0].len();
    let height = image.len();
    let aspect = (width as f32) / (height as f32);
    //let (world, camera) = World::build_random_scene(0.01, f32::MAX, aspect);
    let (world, camera) = World::two_perlin_spheres(0.01, f32::MAX, aspect);
    //let world = build_scene();
    let world = &world;
    println!("Width: {}\nHeight: {}", width, height);
    let bar = ProgressBar::new((height) as u64);
    image.iter_mut().enumerate().for_each(|(j,row)| {
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
