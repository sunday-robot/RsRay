use super::Material;
use crate::types::Vec3;

/// RayとHittableの衝突情報
pub struct HitRecord {
    /// レイ軸上の位置
    pub t: f64,

    /// 衝突点
    pub position: Vec3,

    /// 衝突点の法線
    pub normal: Vec3,

    /// 衝突点の表面素材
    pub material: Box<dyn Material>,

    pub front_face: bool,

    pub u: f64,

    pub v: f64,

    // public HitRecord(double t, Vec3 position, Vec3 normal, Material material, bool frontFace = false, double u = 0, double v = 0)
}

impl HitRecord {
    pub fn new(
        t: f64,
        position: Vec3,
        normal: Vec3,
        material: Box<dyn Material>,
        front_face: bool,
        u: f64,
        v: f64,
    ) -> HitRecord {
        HitRecord {
            t,
            position,
            normal,
            material,
            front_face,
            u,
            v,
        }
    }
}
