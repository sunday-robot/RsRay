use crate::hittables::Sphere;
use crate::materials::Dielectric;
use crate::materials::Lambertian;
use crate::materials::Metal;
use crate::types::Scene;
use crate::types::Camera;
use crate::types::Hittable;
use crate::types::Material;
use crate::types::Rgb;
use crate::types::Vec3;
use crate::utils::rand::*;

pub fn create() -> Scene {
    let mut hittable_vec = Vec::<Box<dyn Hittable>>::new();
    {
        // 多数の小さな級
        for a in (-11)..10 {
            for b in -11..10 {
                let center = Vec3::new(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                    continue;
                }
                let material: Box<dyn Material>;
                let chooseMat = rand();
                if chooseMat < 0.8 {
                    // ざらついたプラスチックのような素材
                    material = Box::new(Lambertian::create_from_rgb2(rand2(), rand2(), rand2()));
                } else if chooseMat < 0.95 {
                    // 金属
                    material = Box::new(Metal::new(
                        Rgb::new(
                            0.5 * (1.0 + rand()),
                            0.5 * (1.0 + rand()),
                            0.5 * (1.0 + rand()),
                        ),
                        0.5 * rand(),
                    ));
                } else {
                    // ガラス
                    material = Box::new(Dielectric::new(1.5));
                }
                hittable_vec.push(Box::new(Sphere::new(center, 0.2, material)));
            }
        }

        // // 三つの大きな球
        // hittables.append(Sphere::new(
        //     Vec3::new(0.0, 1.0, 0.0),
        //     1.0,
        //     Box::new(Dielectric::new(1.5)),
        // ));
        // hittables.append(Sphere::new(
        //     Vec3::new(-4.0, 1.0, 0.0),
        //     1.0,
        //     Box::new(Lambertian::new(Box::new(Rgb::new(0.4, 0.2, 0.1)))),
        // ));
        // hittables.append(Sphere::new(
        //     Vec3::new(4.0, 1.0, 0.0),
        //     1.0,
        //     Box::new(Metal::new(Rgb::new(0.7, 0.6, 0.5), 0)),
        // ));

        // // 地面となる非常に大きな球
        // hittables.append(Sphere::new(
        //     Vec3::new(0.0, -1000.0, 0.0),
        //     1000.0,
        //     Box::new(Lambertian::new(Rgb::new(0.5, 0.5, 0.5))),
        // ));
    }

    let lookFrom = Vec3::new(13.0, 2.0, 3.0);
    let lookAt = Vec3::new(0.0, 0.0, 0.0);
    let vFov = 20.0;
    let aperture = 0.1;
    let exposureTime = 1.0;
    let camera = Camera::create_camera(
        lookFrom,
        lookAt,
        Vec3::new(0.0, 1.0, 0.0),
        vFov,
        16.0 / 9.0,
        aperture,
        exposureTime,
    );

    return Scene::new(hittable_vec, camera, None);
}
