mod vec3;
mod ray;
mod hit;
mod hittables;
mod camera;
mod utils;

use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use vec3::{Colour,Vec3,Point3};
use ray::{Ray, ray_color};
use hittables::sphere::Sphere;
use hit::World;
use camera::Camera;
use utils::clamp;
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    
    // Set up image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH:u32 = 500;
    const IMAGE_HEIGHT:u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL:f64 = 50.0;

    // Camera
    let camera = Camera::new();

    // World
    let mut world = World::new();
    world.add(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0));
    world.add(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5));

    // Render
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let pb = ProgressBar::new(img.len().try_into().unwrap());
    const MAX_DEPTH:i32 = 10;

    for (x,y,pixel) in img.enumerate_pixels_mut() {

        pb.inc(1);

        // Reverse y scanlines
        let y = IMAGE_HEIGHT-y;

        let mut pixel_colour = Colour::new(0.0,0.0,0.0);

        for _n in 0..SAMPLES_PER_PIXEL as i32 {
            let u = (x as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH as f64 -1.0);
            let v = (y as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT as f64 -1.0);
    
            let ray = camera.get_ray(u, v);
            pixel_colour += ray_color(&ray, &world, MAX_DEPTH);

        }

        let mut r = pixel_colour.x as f64; 
        let mut g = pixel_colour.y as f64; 
        let mut b = pixel_colour.z as f64; 

        let scale = 1.0 / SAMPLES_PER_PIXEL;
        r *= scale;
        g *= scale;
        b *= scale;

        let ir = (255.999 * clamp(r, 0.0, 0.999)) as u8;
        let ig  = (255.999 * clamp(g as f64, 0.0, 0.999)) as u8;
        let ib  = (255.999 * clamp(b as f64, 0.0, 0.999)) as u8;

        *pixel = Rgb([ir, ig, ib])
    }

    img.save("image.png").unwrap();
}
