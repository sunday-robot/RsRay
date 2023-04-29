use super::Aabb;

pub trait GetBoundingBoxTrait {
    /// レンダリング前のBVHツリー構築時に一度呼ばれるだけなので、速度についてあまり考慮する必要はない。
    fn bounding_box(&self, exposure_time: f64) -> Aabb;
}
