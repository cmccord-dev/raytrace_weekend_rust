use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub mod sphere;

pub struct Hit<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Box<Material+'a>,
}
pub trait Object: std::marker::Sync {
    fn hits(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
