mod create_bvh_tree;
mod hittables;
mod materials;
mod scene_creators;
mod textures;
mod types;
mod utils;
mod renderer;

use std::time::SystemTime;
use renderer::Renderer;
use types::Hittable;

fn main() {
    println!("Hello, world!");
    let image_with = 1280;
    let image_height = 720;
    let over_sampling_count = 100;

    let scene = scene_creators::scene_0::create();
    let world = create_bvh_tree::create(scene.hittables, scene.camera.exposure_time);
    println!("{:?}", world);
    let start = SystemTime::now();
    let z:Box<dyn Hittable> = &world as Box<dyn Hittable>;
    let renderer = Renderer::new(z, scene.background);
    let pixels = renderer.render(
        scene.camera,
        image_with,
        image_height,
        50,
        over_sampling_count,
    );
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("time = {:?}", duration.as_millis());
    utils::save_as_bmp(image_with, image_height, pixels, "Spheres.bmp");
}
