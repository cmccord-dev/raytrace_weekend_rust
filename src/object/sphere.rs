use crate::Material;
use crate::{Hit, Object};
use crate::Ray;
use crate::Vec3;
use crate::AABB;
use std::f32;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<Material>,    
    bounds: Option<AABB>,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            bounds:Some(AABB::new(center - Vec3::from(radius), center + Vec3::from(radius)))
        }
    }
    fn uv(&self, p: &Vec3) -> (f32, f32) {
        let p = (p - self.center)/self.radius;
        let mut phi = f32::atan2(p.z, p.x);
        if p.x < 0.0 {
            if p.z >= 0.0 {
                phi = phi - f32::consts::PI;
            }else{
                phi = phi + f32::consts::PI;
            }
        }
        let theta = f32::asin(p.y);
        let uv = (1.0 - (phi + f32::consts::PI) / (2.0*f32::consts::PI), (theta+f32::consts::PI/2.0) / f32::consts::PI);
        //println!("{:?},{:?}", phi / f32::consts::PI, uv);
        uv
    }
}
impl Object for Sphere {
    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        self.bounds
    }
    fn hits(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.src - self.center;
        let a = ray.dir.dot(&ray.dir);
        let b = 2.0 * oc.dot(&ray.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discr = b * b - 4.0 * a * c;
        
        if discr >= 0.0 {
            let t = (-b - discr.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                let p = ray.at_parameter(t);
                let (u,v) = self.uv(&p);
                return Some(Hit {
                    t,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material,
                    u,v
                });
            }
            let t = (-b + discr.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                let p = ray.at_parameter(t);
                let (u,v) = self.uv(&p);
                return Some(Hit {
                    t,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material,
                    u,v
                });
            }
        }
        None
    }
}
