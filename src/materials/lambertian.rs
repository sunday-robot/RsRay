use crate::textures::SolidColor;
use crate::types::HitRecord;
use crate::types::Material;
use crate::types::Ray;
use crate::types::Rgb;
use crate::types::Texture;
use crate::types::Vec3;
use crate::utils::rand::*;

/// <summary>
/// 完全な拡散反射をする材質(入射角とは無関係に反射する)
/// </summary>
#[derive(Debug)]
pub struct Lambertian {
    /// <summary>素材の色</summary>
    pub albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn create_from_rgb(rgb: Rgb) -> Lambertian {
        Lambertian {
            albedo: Box::new(SolidColor::new(rgb)),
        }
    }

    pub fn create_from_rgb2(r: f64, g: f64, b: f64) -> Lambertian {
        Lambertian {
            albedo: Box::new(SolidColor::new(Rgb::new(r, g, b))),
        }
    }
}

impl Material for Lambertian {
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Rgb {
        crate::types::rgb::BLACK
    }

    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Rgb, Ray)> {
        let mut scatter_direction = rec.normal + random_in_unit_sphere();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.position, scatter_direction, ray.time);
        let attenuation = self.albedo.value(rec.u, rec.v, rec.position);
        return Some((attenuation, scattered));
    }

    // public override string ToString()
    // {
    //     return $"Lambertian({_albedo})";
    // }
}
