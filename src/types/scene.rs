use super::Camera;
use super::Hittable;
use super::Rgb;

#[derive(Debug)]
pub struct Scene {
    pub hittables: Vec<Box<dyn Hittable>>,
    pub camera: Camera,
    pub background: Option<Rgb>,
}

impl Scene {
    pub fn new(hittables: Vec<Box<dyn Hittable>>, camera:Camera, background:Option<Rgb>) -> Scene {
        Scene { hittables, camera, background} 
    }
}