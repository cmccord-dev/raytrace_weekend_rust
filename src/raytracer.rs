
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
fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.src - center;
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * oc.dot(ray.dir);
    let c = oc.dot(oc) - radius * radius;
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 {
        -1.0
    } else {
        (-b - discr.sqrt()) / (2.0 * a)
    }
}
fn getHit(ray: &Ray, objs: &Vec<Box<Object>>) -> Option<Hit> {
    objs.iter()
        .map(|obj| (*obj).hits(ray, 0.0, f32::MAX))
        .filter_map(|hit| hit)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal))
}
fn color(ray: &Ray, objs: &Vec<Box<Object>>) -> Vec3 {
    if let Some(tmp) = getHit(ray, objs) {
        (tmp.normal + 1.0) * 0.5

    /*let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        ((ray.at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)).norm()+1.0)*0.5*/
    } else {
        let t: f32 = 0.5 * (ray.dir.y + 1.0);
        Vec3::lerp(Vec3::from_value(1.0), Vec3::new(0.5, 0.7, 1.0), t)
    }
}