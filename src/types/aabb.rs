use super::Vec3;
use super::Ray;

/// Axis Aligned Bounding Box
#[derive(Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Aabb {
        Aabb { min, max }
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        let mut a: f64;
        let mut b: f64;
        let mut tmin = t_min;
        let mut tmax = t_max;

        a = (self.min.x - ray.origin.x) / ray.direction.x;
        b = (self.max.x - ray.origin.x) / ray.direction.x;
        if a > b {
            if b > tmin {
                tmin = b;
            }
            if a < tmax {
                tmax = a;
            }
        } else {
            if a > tmin {
                tmin = a;
            }
            if b < tmax {
                tmax = b;
            }
        }
        if tmax <= tmin {
            return false;
        }

        a = (self.min.y - ray.origin.y) / ray.direction.y;
        b = (self.max.y - ray.origin.y) / ray.direction.y;
        if a > b {
            if b > tmin {
                tmin = b;
            }
            if a < tmax {
                tmax = a;
            }
        } else {
            if a > tmin {
                tmin = a;
            }
            if b < tmax {
                tmax = b;
            }
        }
        if tmax <= tmin {
            return false;
        }

        a = (self.min.z - ray.origin.z) / ray.direction.z;
        b = (self.max.z - ray.origin.z) / ray.direction.z;

        if a > b {
            if b > tmin {
                tmin = b;
            }
            if a < tmax {
                tmax = a;
            }
        } else {
            if a > tmin {
                tmin = a;
            }
            if b < tmax {
                tmax = b;
            }
        }
        if tmax <= tmin {
            return false;
        }
        return true;
    }
}

pub fn surrounding_aabb(a: Aabb, b: Aabb) -> Aabb {
    let min_x = f64::min(a.min.x, b.min.x);
    let max_x = f64::max(a.max.x, b.max.x);
    let min_y = f64::min(a.min.y, b.min.y);
    let max_y = f64::max(a.max.y, b.max.y);
    let min_z = f64::min(a.min.z, b.min.z);
    let max_z = f64::max(a.max.z, b.max.z);
    return Aabb::new(
        Vec3::new(min_x, min_y, min_z),
        Vec3::new(max_x, max_y, max_z),
    );
}

// public override string ToString()
// {
//     return $"{Min}-{Max}";
// }
