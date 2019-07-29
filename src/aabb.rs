use std::f32;
use crate::Vec3;
use crate::Ray;


#[derive(Copy,Clone)]
pub struct AABB{
    pub min: Vec3,
    pub max: Vec3,
    pub center: Vec3,
}
impl AABB {
    pub fn new(min:Vec3, max:Vec3) -> Self
    {
        Self {
            min,
            max,
            center:(min+max)/2.0
        }
    }
    #[inline]
    pub fn hit(&self, r:&Ray, tmin:f32, tmax:f32) -> bool {
        
        let inv_d = 1.0 / r.dir.x;
        let t0 = (self.min.x - r.src.x) * inv_d;
        let t1 = (self.max.x - r.src.x) * inv_d;
        let (t0, t1) = if inv_d < 0.0{
             (t1, t0) 
        }else { (t0, t1) };
        let tmin = tmin.min(t0);
        let tmax = tmax.max(t1);
        if tmax <= tmin {
            return false;
        }
        
        let inv_d = 1.0 / r.dir.y;
        let t0 = (self.min.y - r.src.y) * inv_d;
        let t1 = (self.max.y - r.src.y) * inv_d;
        let (t0, t1) = if inv_d < 0.0{
             (t1, t0) 
        }else { (t0, t1) };
        let tmin = tmin.min(t0);
        let tmax = tmax.max(t1);
        if tmax <= tmin {
            return false;
        }
        let inv_d = 1.0 / r.dir.z;
        let t0 = (self.min.z - r.src.z) * inv_d;
        let t1 = (self.max.z - r.src.z) * inv_d;
        let (t0, t1) = if inv_d < 0.0{
             (t1, t0) 
        }else { (t0, t1) };
        let tmin = tmin.min(t0);
        let tmax = tmax.max(t1);
        if tmax <= tmin {
            return false;
        }
        true
    }
    pub fn merge(&self, b:&AABB) -> AABB{
        let min = Vec3::new(
            self.min.x.min(b.min.x),
            self.min.y.min(b.min.y),
            self.min.z.min(b.min.z)
        );
                let max = Vec3::new(
            self.max.x.max(b.max.x),
            self.max.y.max(b.max.y),
            self.max.z.max(b.max.z)
        );
        AABB{
            min,max,center:(min+max)/2.0
        }
    }
    
}