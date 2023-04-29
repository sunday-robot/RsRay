pub mod lambertian;
pub use lambertian::Lambertian;

mod metal;
pub use metal::Metal;

mod dielectric;
pub use dielectric::Dielectric;

mod debug_material;
pub use debug_material::DebugMaterial;
