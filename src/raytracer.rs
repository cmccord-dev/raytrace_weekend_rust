
use crate::image::Image;
use crate::object::sphere::Sphere;
use crate::ray::Ray;
use crate::object::{Hit, Object};
use crate::vec3::Vec3;

use std::f32;
pub fn raytrace(image: &mut Image) {
    let width = image[0].len();
    let height = image.len();
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::from_value(0.0);

    let mut objs: Vec<Box<Object>> = Vec::new();
    objs.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    objs.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    println!("Width: {}\nHeight: {}", width, height);
    for (j, row) in image.iter_mut().enumerate() {
        for (i, vec) in row.iter_mut().enumerate() {
            let u = (i as f32) / (width as f32);
            let v = ((height - j) as f32) / (height as f32);
            let ray = Ray::new(origin, lower_left + horizontal * u + vertical * v);
            *vec = color(&ray, &objs);

        }
    }
}
fn get_hit(ray: &Ray, objs: &Vec<Box<Object>>) -> Option<Hit> {
    objs.iter()
        .map(|obj| (*obj).hits(ray, 0.0, f32::MAX))
        .filter_map(|hit| hit)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal))
}
fn color(ray: &Ray, objs: &Vec<Box<Object>>) -> Vec3 {
    if let Some(tmp) = get_hit(ray, objs) {
        (tmp.normal + 1.0) * 0.5
    } else {
        let t: f32 = 0.5 * (ray.dir.y + 1.0);
        Vec3::lerp(Vec3::from_value(1.0), Vec3::new(0.5, 0.7, 1.0), t)
    }
}