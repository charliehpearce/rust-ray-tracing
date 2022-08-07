mod camera;
mod hittables;
mod materials;
mod ray;
mod utils;
mod vec3;

use camera::Camera;
use hittables::lib::World;
use hittables::sphere::Sphere;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::{thread_rng, Rng};
use ray::ray_colour;
use utils::clamp;
use vec3::{Colour, Point3};

use crate::materials::{Lambertian, Metal};

fn main() {
    let mut rng = thread_rng();

    // Set up image and camera
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 500;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const APERTURE: f64 = 0.1;
    const LOOKFROM: Point3 = Point3::new(13.0, 2.0, 3.0);
    const LOOKAT: Point3 = Point3::new(0.0, 0.0, 0.0);
    const VUP: Point3 = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;

    let camera = Camera::new(
        LOOKFROM,
        LOOKAT,
        VUP,
        20.0,
        ASPECT_RATIO,
        APERTURE,
        dist_to_focus,
    );

    // World
    let mut world = World::new();
    // Define materials
    let material_ground: Lambertian = materials::Lambertian::new(Colour::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if choose_mat < 0.8 {
                let albedo = Colour::new(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                );
                let sphere_material = materials::Lambertian::new(albedo);
                world.add(Sphere::new(center, 0.2, sphere_material));
            } else {
                let albedo = Colour::new(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                );
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_material = materials::Metal::new(albedo, fuzz);
                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    // Render
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    const MAX_DEPTH: i32 = 20;
    const SAMPLES_PER_PIXEL: f64 = 100.0;

    let pb = ProgressBar::new(img.pixels().len().try_into().unwrap());

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        pb.inc(1);

        // Reverse y scanlines
        let y = IMAGE_HEIGHT - y;

        let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);

        for _n in 0..SAMPLES_PER_PIXEL as i32 {
            let u = (x as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH as f64 - 1.0);
            let v = (y as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT as f64 - 1.0);

            let ray = camera.get_ray(u, v);
            pixel_colour += ray_colour(&ray, &world, MAX_DEPTH);
        }

        let mut r = pixel_colour.x as f64;
        let mut g = pixel_colour.y as f64;
        let mut b = pixel_colour.z as f64;

        let scale = 1.0 / SAMPLES_PER_PIXEL;
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        let ir = (255.999 * clamp(r, 0.0, 0.999)) as u8;
        let ig = (255.999 * clamp(g as f64, 0.0, 0.999)) as u8;
        let ib = (255.999 * clamp(b as f64, 0.0, 0.999)) as u8;

        *pixel = Rgb([ir, ig, ib])
    }

    img.save("image.png").unwrap();
}
