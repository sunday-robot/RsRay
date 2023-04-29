use super::HitRecord;
use super::Ray;
use super::GetBoundingBoxTrait;

pub trait Hittable: std::fmt::Debug + GetBoundingBoxTrait {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
