use super::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    /// 起点
    pub origin: Vec3,

    /// 方向
    pub direction: Vec3,

    pub time: f64,
}

impl Ray {
    pub fn new(_origin: Vec3, _direction: Vec3, _time: f64) -> Ray {
        Ray {
            origin: _origin,
            direction: _direction,
            time: _time,
        }
    }

    pub fn position_at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
