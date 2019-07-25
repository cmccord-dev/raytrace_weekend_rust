use crate::ray::Ray;
use crate::vec3::Vec3;
pub mod sphere;
pub struct Hit {
    pub t:f32,
    pub p:Vec3,
    pub normal:Vec3,
}
pub trait Object {
    fn hits(&self, ray:&Ray, t_min:f32, t_max:f32) -> Option<Hit>;
}