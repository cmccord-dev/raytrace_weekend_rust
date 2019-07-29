use crate::Hit;
use crate::Vec3;
use crate::Ray;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material: std::marker::Sync {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter>;
}
