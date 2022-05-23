mod vec3;
mod ray;
mod utils;
mod hittables;

pub use crate::vec3::{Vec3,Colour,Point3};
pub use crate::ray::Ray;
pub use crate::hittables::sphere::Sphere;
pub use crate::hittables::hit::HittableList;

use image::{ImageBuffer, Rgb, RgbImage};

fn main() {
    
    // Set up Image
    const ASPECT_RATIO:f64 = 16.0/9.0;
    const IMAGE_WIDTH:u32 = 2000;
    const IMAGE_HEIGHT:u32 = (IMAGE_WIDTH as f64/ ASPECT_RATIO) as u32;

    // Camera
    const VIEWPORT_HEIGHT:f64 = 2.0;
    const VIEWPORT_WIDTH:f64 = ASPECT_RATIO*VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;
    
    let ORIGIN:Point3 = Point3::new(0.0,0.0,0.0);
    let horizontal:Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let veritcal: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = ORIGIN - horizontal/2.0 - veritcal/2.0 - Vec3::new(0.0,0.0,FOCAL_LENGTH);

    // World
    let mut world: HittableList = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0,-105.0,-1.0), 100.0));
    
    // Render

    let mut buffer : RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x,y,pixel) in buffer.enumerate_pixels_mut() {

        let v = y as f64 / (IMAGE_HEIGHT as f64 +1.0);
        let u = x as f64 / (IMAGE_WIDTH as f64+1.0);
        let direction:Vec3 = lower_left_corner + horizontal*u + veritcal*v;
        let ray:Ray = Ray::new(ORIGIN, direction);
        
        let ray_colour:Colour = ray.ray_colour(&world);

        let ir = (ray_colour.x() * 255.999) as u8;
        let ig = (ray_colour.y() * 255.999) as u8;
        let ib = (ray_colour.z() * 255.999) as u8;

        *pixel = Rgb([ir, ig, ib]);
        
    }

    buffer.save("image.png").unwrap();
}
