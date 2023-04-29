use crate::types::Camera;
use crate::types::Hittable;
use crate::types::Ray;
use crate::types::Rgb;
use crate::utils::rand::rand;

const T_MIN: f64 = 0.001;

#[derive(Debug)]
pub struct Renderer {
    pub world: Box<dyn Hittable>,
    pub background: Option<Rgb>,
}

impl Renderer {
    pub fn new(world: Box<dyn Hittable>, background: Option<Rgb>) -> Renderer {
        Renderer { world, background }
    }

    pub fn render(
        &self,
        camera: Camera,
        width: i32,
        height: i32,
        max_depth: i32,
        sample_count: i32,
    ) -> Vec<Rgb> {
        let pixels = Vec::<Rgb>::with_capacity((height * width) as usize);
        // 並列処理を行わない(39.2秒)
        for y in 0..height {
            println!("{0}/{1}", y, height);
            for x in 0..width {
                let mut rgb_sum = Rgb::new(0.0, 0.0, 0.0);
                for i in 0..sample_count {
                    let u = (x as f64 + rand()) / (width as f64);
                    let v = ((height as f64 - 1.0) - (y as f64 + rand())) / (height as f64);
                    let r = camera.get_ray(u, v);
                    let rgb = self.color(r, max_depth);
                    rgb_sum = rgb_sum + rgb;
                }
                pixels[(y * width + x) as usize] = rgb_sum / (sample_count as f64);
            }
        }
        return pixels;
    }

    /**
     * 色を返す
     * @param ray レイ
     * @param world 物体群
     * @param depth 残りの追跡回数
     * @return 色
     */
    fn color(&self, ray: Ray, depth: i32) -> Rgb {
        // 追跡回数が規定値に達した場合は(0,0,0)を返す
        if depth <= 0 {
            return crate::types::rgb::BLACK;
        }

        let rec = self.world.hit(ray, T_MIN, f64::MAX);
        match rec {
            // どの物体ともヒットしない場合は、指定された背景色あるいは天球の色を返す
            None => match self.background {
                None => {
                    let unit_direction = ray.direction.unit();
                    let t = 0.5 * (unit_direction.y + 1.0);
                    let v1 = Rgb::new(1.0, 1.0, 1.0);
                    let v2 = Rgb::new(0.5, 0.7, 1.0);
                    (1.0 - t) * v1 + t * v2
                }
                Some(rgb) => return rgb,
            },
            Some(hit_record) => {
                let emitted =
                    hit_record
                        .material
                        .emitted(hit_record.u, hit_record.v, hit_record.position);
                let s = hit_record.material.scatter(ray, hit_record);
                match s {
                    None => emitted,
                    Some(ss) => emitted + ss.0 * self.color(ss.1, depth - 1),
                }
            }
        }
    }
}
