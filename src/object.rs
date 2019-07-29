use crate::Material;
use crate::Ray;
use crate::Vec3;
use crate::AABB;
pub mod sphere;

pub struct Hit<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Box<Material + 'a>,
}
pub trait Object: std::marker::Sync {
    fn hits(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
