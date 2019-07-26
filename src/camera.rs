use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(left_corner: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Camera {
        Camera {
            left_corner,
            horizontal,
            vertical,
            origin,
        }
    }
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(
            self.origin,
            self.left_corner + u * self.horizontal + v * self.vertical - self.origin,
        );
    }
}
