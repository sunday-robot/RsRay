use crate::types::material::reflect;
use crate::types::rgb::BLACK;
use crate::types::HitRecord;
use crate::types::Material;
use crate::types::Ray;
use crate::types::Rgb;
use crate::types::Vec3;
use crate::utils::rand::rand;

/// 誘電体マテリアル(透明な材質)
#[derive(Debug)]
pub struct Dielectric {
    /// 屈折率
    pub refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Rgb {
        BLACK
    }

    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Rgb, Ray)> {
        let attenuation = Rgb::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.direction.unit();
        let dt = unit_direction.dot(rec.normal);
        let cos_theta = f64::min(-dt, 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        //var cannotRefract = refractionRatio * sinTheta > 1;
        let direction: Vec3;
        if refraction_ratio * sin_theta > 1.0 {
            direction = reflect(unit_direction, rec.normal);
        } else {
            if reflectance(cos_theta, refraction_ratio) > rand() {
                direction = reflect(unit_direction, rec.normal);
            } else {
                // #if false
                //             direction = Refract(unitDirection, rec.Normal, refractionRatio);
                // #else
                let discriminant = 1.0 - refraction_ratio * refraction_ratio * (1.0 - dt * dt);
                if discriminant <= 0.0 {
                    return None;
                }
                direction = refraction_ratio * (unit_direction - rec.normal * dt)
                    - rec.normal * f64::sqrt(discriminant);
                // #endif
            }
        }
        let scattered = Ray::new(rec.position, direction, ray.time);

        return Some((attenuation, scattered));
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r02 = r0 * r0;
    return r0 + (1.0 - r02) * (1.0 - cosine).powf(5.0);
}

// public override string ToString()
// {
//     return $"Dielectric({_refractiveIndex})";
// }
