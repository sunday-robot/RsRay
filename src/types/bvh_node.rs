use super::Aabb;
use super::Bvh;
use super::BvhTrait;
use super::HitRecord;
use super::Hittable;
use super::Ray;
use super::GetBoundingBoxTrait;

#[derive(Debug)]
pub struct BvhNode {
    base: Bvh,
    pub left: Box<dyn BvhTrait>,
    pub right: Box<dyn BvhTrait>,
}

impl BvhNode {
    pub fn new(aabb: Aabb, left: Box<dyn BvhTrait>, right: Box<dyn BvhTrait>) -> BvhNode {
        BvhNode {
            base: Bvh::new(aabb),
            left,
            right,
        }
    }
}

impl GetBoundingBoxTrait for BvhNode {
    fn bounding_box(&self, exposure_time: f64) -> Aabb {
        self.base.bounding_box(exposure_time)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.base.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        match self.left.hit(ray, t_min, t_max) {
            None => self.right.hit(ray, t_min, t_max),
            Some(rec1) => match self.right.hit(ray, t_min, rec1.t) {
                None => Some(rec1),
                Some(rec2) => Some(rec2),
            },
        }
    }
}

impl BvhTrait for BvhNode {
    // fn upcast(&self) -> dyn Hittable {

    // }

    // fn upcast(&self) -> dyn Hittable {
    //     self
    // }
}

// public override void Print(string indent = "")
// {
//     Console.WriteLine($"{indent}aabb = {_aabb}");
//     Console.WriteLine($"{indent}left:");
//     _left.Print(indent + "  ");
//     Console.WriteLine($"{indent}right:");
//     _right.Print(indent + "  ");
// }
