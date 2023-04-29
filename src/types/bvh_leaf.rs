use super::bvh;
use super::Aabb;
use super::Bvh;
use super::BvhTrait;
use super::HitRecord;
use super::Hittable;
use super::Ray;
use super::GetBoundingBoxTrait;

#[derive(Debug)]
pub struct BvhLeaf {
    pub base: Bvh,
    pub hittable: Box<dyn Hittable>,
}

impl BvhLeaf {
    pub fn new(aabb: Aabb, hittable: Box<dyn Hittable>) -> BvhLeaf {
        BvhLeaf {
            base: Bvh::new(aabb),
            hittable,
        }
    }
}

impl GetBoundingBoxTrait for BvhLeaf {
    fn bounding_box(&self, exposure_time: f64) -> Aabb {
        self.base.bounding_box(exposure_time)
    }
}

impl Hittable for BvhLeaf {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.base.aabb.hit(ray, t_min, t_max) {
            return None;
        }
        if self.base.debug_mode {
            return Some(HitRecord::new(
                0.0,
                bvh::DUMMY_VEC3,
                bvh::DUMMY_VEC3,
                bvh::DUMMY_MATERIAL,
                false,
                0.0,
                0.0,
            ));
        }

        return self.hittable.hit(ray, t_min, t_max);
    }
}

impl BvhTrait for BvhLeaf {
    // fn upcast(&self) ->( &dyn Hittable ){
    //     // self.hit(ray, t_min, t_max)
    //     // let a = Box::new(self as Hittable);
    //     // a
    // }
}

// public override void Print(string indent = "")
// {
//     Console.WriteLine($"{indent}aabb = {_aabb}");
//     Console.WriteLine($"{indent}{_hittable}");
// }
