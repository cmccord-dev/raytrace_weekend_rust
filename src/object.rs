use crate::Material;
use crate::Ray;
use crate::Vec3;
use crate::AABB;
pub mod sphere;

pub struct Hit<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub u: f32,
    pub v: f32,
    pub material: &'a Box<Material + 'a>,
}
pub trait Object: std::marker::Sync {
    fn hits(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

pub struct DummyObject {
    pub bounds: AABB
}

impl DummyObject {
    pub fn new(bounds:AABB) -> Self {
        Self {
            bounds
        }
    }
}
impl Object for DummyObject {

    fn hits(&self, _ray: &Ray, _t_min: f32, _t_max: f32) -> Option<Hit> {
        None
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB>{
        Some(self.bounds)
        //None
    }
}
