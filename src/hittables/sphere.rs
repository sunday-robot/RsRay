use crate::types::GetBoundingBoxTrait;
use crate::types::HitRecord;
use crate::types::Hittable;
use crate::types::Material;
use crate::types::Ray;
use crate::types::Vec3;
use crate::types::Aabb;

#[derive(Debug)]
pub struct Sphere {
    /// 位置
    pub center: Vec3,

    /// <summary>半径</summary>
    pub radius: f64,

    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl GetBoundingBoxTrait for Sphere {
    fn bounding_box(&self, exposure_time: f64) -> crate::types::Aabb {
        let min = self.center - Vec3::new(self.radius, self.radius, self.radius);
        let max = self.center + Vec3::new(self.radius, self.radius, self.radius);
        return Aabb::new(min, max);
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let half_b = oc.dot(ray.direction);
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let d2 = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - d2) / a;
        if root < t_min || t_max < root {
            root = (-half_b + d2) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.position_at(root);
        let outwardNormal = (p - self.center) / self.radius;
        let (u, v) = get_sphere_uv(outwardNormal);
        let ff = ray.direction.dot(outwardNormal) < 0.0;
        let n = if ff { outwardNormal } else { -outwardNormal };
        return Some(HitRecord::new(root, p, n, self.material, ff, u, v));
    }

}

fn get_sphere_uv(p: Vec3) -> (f64, f64) {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    let theta = f64::acos(-p.y);
    let phi = f64::atan2(-p.z, p.x) + std::f64::consts::PI;
    let u = phi / (2.0 * std::f64::consts::PI);
    let v = theta / std::f64::consts::PI;
    return (u, v);
}

//         public override string ToString()
//         {
//             return $"Sphere(c:{_center}, r:{_radius}, m:{_material})";
//         }
//     }
// }
