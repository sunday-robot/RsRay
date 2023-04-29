use crate::types::Rgb;
use crate::types::Vec3;
use rand::Rng;

pub fn rand() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn rand2() -> f64 {
    return rand() * rand();
}

pub fn rand_i32(max_plus_1: i32) -> i32 {
    rand::thread_rng().gen_range(0..max_plus_1)
}

/// <returns>原点を中心とする半径1のXY平面上の円の中のランダムな位置</returns>
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(2.0 * rand() - 1.0, 2.0 * rand() - 1.0, 0.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

/// <returns>原点を中心とする半径1の球内のランダムな座標</returns>
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand(), rand(), rand()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn random_saturated_rgb(s: f64, v: f64) -> Rgb {
    let min = (1.0 - s) * v;
    let range = v - min;
    let h6 = rand() * 6.0;
    if h6 < 1.0 {
        let g = h6 * range + min;
        return Rgb::new(v, g, min);
    }
    if h6 < 2.0 {
        let r = (2.0 - h6) * range + min;
        return Rgb::new(r, v, min);
    }
    if h6 < 3.0 {
        let b = (h6 - 2.0) * range + min;
        return Rgb::new(min, v, b);
    }
    if h6 < 4.0 {
        let g = (4.0 - h6) * range + min;
        return Rgb::new(min, g, v);
    }
    if h6 < 5.0 {
        let r = (h6 - 4.0) * range + min;
        return Rgb::new(r, min, v);
    }
    let b = (6.0 - h6) * range + min;
    return Rgb::new(v, min, b);
}
