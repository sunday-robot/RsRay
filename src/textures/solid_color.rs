use crate::types::{Rgb, Texture};

#[derive(Debug)]
pub struct SolidColor {
    pub rgb: Rgb,
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: crate::types::Vec3) -> Rgb {
        self.rgb
    }
}

impl SolidColor {
    pub fn new(rgb: Rgb) -> SolidColor {
        SolidColor { rgb }
    }

    pub fn create_from_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            rgb: Rgb::new(r, g, b),
        }
    }
}
