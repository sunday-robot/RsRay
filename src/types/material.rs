use crate::types::HitRecord;
use crate::types::Ray;
use crate::types::Rgb;
use crate::types::Vec3;

pub trait Material: std::fmt::Debug {
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Rgb;

    /// <param name="ray">レイ</param>
    /// <returns>衝突点で分散(？)されたレイ</returns>
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Rgb, Ray)>;
}

/// <param name="v">入射ベクトル</param>
/// <param name="normal">法線ベクトル</param>
/// <returns>反射ベクトル</returns>
pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    return v - 2.0 * v.dot(normal) * normal;
}

/// <param name="v">入射ベクトル</param>
/// <param name="normal">法線ベクトル</param>
/// <param name="niOverNt">?</param>
/// <returns>屈折ベクトル(屈折しない場合はnull)</returns>
pub fn refract(v: Vec3, normal: Vec3, niOverNt: f64) -> Option<Vec3> {
    let uv = v.unit();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - niOverNt * niOverNt * (1.0 - dt * dt);
    if discriminant <= 0.0 {
        return None;
    }
    return Some(niOverNt * (uv - normal * dt) - normal * discriminant.sqrt());
}
