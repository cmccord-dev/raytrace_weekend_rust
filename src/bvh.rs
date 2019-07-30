use crate::Object;
use crate::AABB;
use crate::Hit;
use crate::Ray;
use crate::DummyObject;

pub struct BVH {
    pub left: Box<Object>,
    pub right: Box<Object>,
    pub bounds: AABB,
}

impl Object for BVH {
    fn hits(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>{
        if self.bounds.hit(ray, t_min, t_max) {
            let left = self.left.hits(ray,t_min,t_max);
            let right = self.right.hits(ray,t_min,t_max);
            match (left,right) {
                (Some(left), Some(right)) => {
                    if left.t < right.t {
                        Some(left)
                    } else {
                        Some(right)
                    }
                },
                (None, Some(right)) => Some(right),
                (Some(left), None) => Some(left),
                (None, None) => None,
            }
        }else {
            None
        }
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB>{
        Some(self.bounds)
    }
}



impl BVH {
    /*pub fn new(world:&'a mut World, t0:f32, t1:f32) -> Box<BVH<'a>> {
        BVH::new_vec(&mut world.objs.iter().collect::<Vec<&'a Box<Object+'a>>>(), &mut world.objs, t0, t1, 0)
    }*/
    pub fn new(objs: Vec<Box<Object>>, t0: f32, t1: f32) -> Box<BVH> {
        BVH::new_helper(objs, t0, t1, 0)
    }
    fn new_helper(objs: Vec<Box<Object>>, t0: f32, t1: f32, axis:i32) -> Box<BVH> {
        let mut objs = objs;
        match objs.len() {
            1=> {
            let left = objs.remove(0);
            Box::new(BVH{
                right:Box::new(DummyObject::new(left.bounding_box(t0, t1).unwrap())),
                bounds:left.bounding_box(t0, t1).unwrap(),
                left:left,

            })}
            ,
            2=>{
                let left = objs.remove(0);
                let right = objs.remove(0);
                Box::new(BVH{
                bounds:left.bounding_box(t0, t1).unwrap().merge(&right.bounding_box(t0, t1).unwrap()),
                left:left,
                right:right,
            })
            },
            _=>{
                
                objs.sort_unstable_by(|a,b| {
                    a.bounding_box(t0,t1).unwrap().center.axis(axis).partial_cmp(&b.bounding_box(t0,t1).unwrap().center.axis(axis)).unwrap()
                });
                let mid = objs.len()/2;
                let left = objs.split_off(mid);
                let left = BVH::new_helper(left, t0,t1, (axis + 1) % 3) as Box<Object>;
                let right = BVH::new_helper(objs, t0,t1, (axis + 1) % 3) as Box<Object>;
                Box::new(BVH{
                    bounds:left.bounding_box(t0,t1).unwrap().merge(&right.bounding_box(t0,t1).unwrap()),
                    left:left,
                    right:right,
                })
            }
        }
    }
}