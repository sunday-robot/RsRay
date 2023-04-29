use super::Aabb;
use super::GetBoundingBoxTrait;
use super::Hittable;
use super::Material;
use super::Vec3;
use crate::materials::DebugMaterial;

pub const DUMMY_VEC3: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const DUMMY_MATERIAL: Box<dyn Material> = Box::new(DebugMaterial::new());

#[derive(Debug)]
pub struct Bvh {
    pub debug_mode: bool,
    pub aabb: Aabb,
    pub debug_material: Box<dyn Material>,
}

pub trait BvhTrait: Hittable {
    // fn upcast(&self) -> dyn Hittable;
}

impl Bvh {
    pub fn new(aabb: Aabb) -> Bvh {
        Bvh {
            debug_mode: false,
            aabb,
            debug_material: Box::new(DebugMaterial::new()),
        }
    }
}

impl GetBoundingBoxTrait for Bvh {
    fn bounding_box(&self, exposure_time: f64) -> Aabb {
        self.aabb
    }
}

// TODO Bvhは抽象基底クラスで、Hittableインターフェイスのうち、bounding_box()については実装するが、hit()については実装しないということをしたいが、そのようなことはできないらしい。
// impl Hittable for Bvh {
//     fn bounding_box(&self, exposure_time: f64) -> Aabb {
//         self.aabb
//     }
// }

// impl BvhTrait for Bvh {
//     fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

//     }

//     // public abstract void Print(string indent = "");
// }
