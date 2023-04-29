use super::Ray;
use super::Vec3;
use crate::utils::rand::*;

#[derive(Debug)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    /// 視点
    origin: Vec3,

    /// カメラのX軸方向(単位ベクトル)
    u: Vec3,

    /// カメラのY軸方向(単位ベクトル)
    v: Vec3,

    /// レンズのサイズ(ボケを決めるもので、小さいほどボケない。0なら全くボケない。)
    lens_radius: f64,

    /// 露光時間
    pub exposure_time: f64,
}

impl Camera {
    pub fn new(
        _lower_left_corner: Vec3,
        _horizontal: Vec3,
        _vertical: Vec3,
        _origin: Vec3,
        _u: Vec3,
        _v: Vec3,
        _lens_radious: f64,
        _exposure_time: f64,
    ) -> Camera {
        Camera {
            lower_left_corner: _lower_left_corner,
            vertical: _vertical,
            horizontal: _horizontal,
            origin: _origin,
            u: _u,
            v: _v,
            lens_radius: _lens_radious,
            exposure_time: _exposure_time,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let o = self.origin + offset;
        let d = self.lower_left_corner + s * self.horizontal + t * self.vertical - o;
        let time = (rand() - 0.5) * self.exposure_time;
        return Ray::new(o, d, time);
    }
    /// <param name="lookFrom">視点</param>
    /// <param name="lookAt">注視点(視線の方向を決めるためだけのもので、ピントがあう場所ではない。ピント位置は、focusDistで指定する。)</param>
    /// <param name="vup">上方向を示すベクトル(視点、注視点のベクトルと同じ方向でなければよい。直交している必要もないし、長さも適当でよい)</param>
    /// <param name="verticalFov">縦方向の視野(角度[°]]</param>
    /// <param name="aspect">縦横比(幅/高さ)</param>
    /// <param name="aperture">絞り(ボケ具体を決めるもの。0なら全くボケない。)</param>
    /// <param name="focusDist">視点からピントが合う位置までの距離</param>
    /// <param name="time0"></param>
    /// <param name="time1"></param>
    pub fn create_camera(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vertical_fov: f64,
        aspect: f64,
        aperture: f64,
        exposure_time: f64,
    ) -> Camera {
        let theta = vertical_fov * std::f64::consts::PI / 180.0;
        let halfHeight = (theta / 2.0).tan();
        let halfWidth = aspect * halfHeight;
        let w = (look_at - look_from).unit();
        let u = -vup.cross(w).unit();
        let v = -w.cross(u);
        let focusDist = (look_at - look_from).length();
        let lowerLeftCorner =
            look_from + focusDist * w - halfWidth * focusDist * u - halfHeight * focusDist * v;
        let horizontal = 2.0 * halfWidth * focusDist * u;
        let vertical = 2.0 * halfHeight * focusDist * v;
        let lensRadius = aperture / 2.0;
        return Camera::new(
            lowerLeftCorner,
            horizontal,
            vertical,
            look_from,
            u,
            v,
            lensRadius,
            exposure_time,
        );
    }
}
