use crate::object::Hit;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter>;
}
