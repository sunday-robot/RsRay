use crate::types::rgb::BLACK;
use crate::types::Material;
use crate::types::Rgb;
use crate::types::Vec3;
use crate::types::HitRecord;
use crate::types::Ray;
use crate::utils::rand::random_in_unit_sphere;
use crate::types::material::reflect;

/// 金属マテリアル
#[derive(Debug)]
pub struct Metal {
    /// 色
    pub albedo: Rgb,

    /// <=1
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Rgb, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Rgb {
        BLACK
    }

    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Rgb, Ray)> {
        let reflection_direction = reflect(ray.direction.unit(), rec.normal);
        let scattered = Ray::new(
            rec.position,
            reflection_direction + self.fuzz * random_in_unit_sphere(),
            ray.time,
        );
        if scattered.direction.dot(rec.normal) <= 0.0 {
            return None;
        }
        return Some((self.albedo, scattered));
    }

    // public override string ToString()
    // {
    //     return $"Metal({_albedo}, fuzz:{_fuzz:0.000})";
    // }
}
