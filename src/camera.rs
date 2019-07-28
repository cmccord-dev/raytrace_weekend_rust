use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f32;
pub struct Camera {
    left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    radius: f32,
}

impl Camera {
    pub fn new(
        from: Vec3,
        to: Vec3,
        up: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus: f32,
    ) -> Camera {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (from - to).norm();
        let u = up.cross(&w).norm();
        let v = w.cross(&u);
        let radius = aperture / 2.0;
        Camera {
            left_corner: from - half_width * focus * u - half_width * focus * v - focus * w,
            horizontal: 2.0 * half_width * focus * u,
            vertical: 2.0 * half_height * focus * v,
            origin: from,
            u,
            v,
            w,
            radius,
        }
    }
    pub fn ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.radius * Vec3::random_in_unit_disk();
        //let rd = self.radius*Vec3;
        let offset = self.u * rd.x + self.v * rd.y;
        //let offset = 0.0;
        Ray::new(
            self.origin + offset,
            self.left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
        /*return Ray::new(
            self.origin,
            self.left_corner + u * self.horizontal + v * self.vertical - self.origin,
        );*/
    }
}
