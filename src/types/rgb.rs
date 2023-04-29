#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,

}

pub static BLACK:Rgb = Rgb::new(0.0, 0.0, 0.0);

impl Rgb {
    pub fn new(r: f64, g: f64, b: f64) -> Rgb {
        Rgb { r, g, b }
    }
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

/// RGB + RGB
impl std::ops::Add for Rgb {
    type Output = Rgb;

    fn add(self, other: Rgb) -> Rgb {
        Rgb::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

/// RGB * RGB
impl std::ops::Mul<Rgb> for Rgb {
    type Output = Rgb;

    fn mul(self, other: Rgb) -> Rgb {
        Rgb::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

/// RGB * 実数
impl std::ops::Mul<f64> for Rgb {
    type Output = Rgb;

    fn mul(self, other: f64) -> Rgb {
        Rgb::new(self.r * other, self.g * other, self.b * other)
    }
}

/// 実数 * RGB
impl std::ops::Mul<Rgb> for f64 {
    type Output = Rgb;

    fn mul(self, other: Rgb) -> Rgb {
        other * self
    }
}

/// RGB / 実数
impl std::ops::Div<f64> for Rgb {
    type Output = Rgb;

    fn div(self, other: f64) -> Rgb {
        Rgb::new(self.r / other, self.g / other, self.b / other)
    }
}

// 本アプリでは(多分多くのレンダリングプログラムも同様と思われる)光は3次元ベクトルと同様に3つの実数からなるものとしているが、ベクトルのような以下のような演算は定義できない。
// ・内積
// ・外積
// ・減算
// ・符号反転
