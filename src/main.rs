use image::{ImageBuffer, Rgb, RgbImage};

fn main() {
    // Set up image parameters
    const IMAGE_WIDTH:u32 = 1024;
    const IMAGE_HEIGHT:u32 = 1024;

    let mut buffer : RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x,y,pixel) in buffer.enumerate_pixels_mut() {
        let r = x as f64 / (IMAGE_WIDTH-1) as f64;
        let g = y as f64 / (IMAGE_WIDTH-1) as f64;
        let b = 0.25;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        *pixel = Rgb([ir, ig, ib]);
        
    }

    buffer.save("image.png").unwrap();
}
