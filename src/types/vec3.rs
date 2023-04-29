#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(_x: f64, _y: f64, _z: f64) -> Vec3 {
        Vec3 {
            x: _x,
            y: _y,
            z: _z,
        }
    }

    /// 長さの2乗
    pub fn squared_length(&self) -> f64 {
        self.dot(*self)
    }

    /// 長さ
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    /// 単位ベクトル(向きが同じで長さが1のベクトル)
    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    /// 事実上(0, 0, 0)とみなせるかどうか
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return (num::Float::abs(self.x) < s)
            && (num::Float::abs(self.y) < s)
            && (num::Float::abs(self.z) < s);
    }

    /// 内積
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// 外積
    pub fn cross(&self, other: Vec3) -> Vec3 {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vec3::new(x, y, z)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) -> () {
        *self = Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z);
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        self + (-other)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) -> () {
        *self = Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z);
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.x / other, self.y / other, self.z / other)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
