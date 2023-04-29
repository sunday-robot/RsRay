use crate::types::HitRecord;
use crate::types::Material;
use crate::types::Ray;
use crate::types::Rgb;
use crate::types::Vec3;
use crate::utils::rand::random_saturated_rgb;

#[derive(Debug)]
pub struct DebugMaterial {
    pub color: Rgb,
}

impl DebugMaterial {
    pub fn new() -> DebugMaterial {
        DebugMaterial {
            color: random_saturated_rgb(1.0, 1.0),
        }
    }
}

impl Material for DebugMaterial {
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Rgb {
        self.color
    }

    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Rgb, Ray)> {
        None
    }
}
