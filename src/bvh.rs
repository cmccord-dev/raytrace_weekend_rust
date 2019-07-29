use crate::Object;
use crate::Vec3;
use crate::AABB;
use crate::Hit;
use crate::Ray;
use crate::World;
use rand::Rng;

pub struct BVH<'a> {
    pub left: &'a Box<Object+'a>,
    pub right: &'a Box<Object+'a>,
    pub bounds: AABB,
}

impl Object for BVH<'_> {
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
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>{
        Some(self.bounds)
    }
}



impl BVH<'_> {
    /*pub fn new(world:&'a mut World, t0:f32, t1:f32) -> Box<BVH<'a>> {
        BVH::new_vec(&mut world.objs.iter().collect::<Vec<&'a Box<Object+'a>>>(), &mut world.objs, t0, t1, 0)
    }*/
    fn new_vec(list:&mut Vec<&Box<Object>>, objs: &mut Vec<Box<Object>>, t0:f32, t1:f32, axis:i32) -> Box<BVH> {
        match list.len() {
            1=> Box::new(BVH{
                left:list[0],
                right:list[0],
                bounds:list[0].bounding_box(t0, t1).unwrap()
            }),
            2=>Box::new(BVH{
                left:list[0],
                right:list[1],
                bounds:list[0].bounding_box(t0, t1).unwrap().merge(&list[1].bounding_box(t0, t1).unwrap())
            }),
            _=>{
                
                list.sort_unstable_by(|a,b| {
                    a.bounding_box(t0,t1).unwrap().center.axis(axis).partial_cmp(&b.bounding_box(t0,t1).unwrap().center.axis(axis)).unwrap()
                });
                let mid = list.len()/2;
                let mut left = list.split_off(mid);
                let left = BVH::new_vec(&mut left, objs, t0,t1, (axis + 1) % 3) as Box<Object>;
                let right = BVH::new_vec(list, objs, t0,t1, (axis + 1) % 3) as Box<Object>;
                let i = objs.len();
                objs.push(left);
                objs.push(right);
                Box::new(BVH{
                    left:&objs[i],
                    right:&objs[i+1],
                    bounds:left.bounding_box(t0,t1).unwrap().merge(&right.bounding_box(t0,t1).unwrap())
                })
            }
        }
    }
}