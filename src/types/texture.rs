use super::Rgb;
use super::Vec3;

pub trait Texture: std::fmt::Debug {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Rgb;
}
