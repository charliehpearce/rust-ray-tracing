use std::ops::Range;

pub use nalgebra::Vector3;
use rand::{thread_rng, Rng};

pub type Vec3 = Vector3<f64>;
pub type Point3 = Vec3;
pub type Colour = Vec3;

pub fn near_zero(x: f64, y: f64, z: f64) -> bool {
    let e = 1e-8;
    return (x < e) && (y < e) && (z < e);
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let unit = Vec3::new(1.0, 1.0, 0.0);
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - unit;
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

fn random_rng_vec3(range: Range<f64>) -> Vec3 {
    let mut rng = thread_rng();
    return Vec3::new(
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
    );
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_rng_vec3(-1.0..1.0);

        if p.dot(&p) >= 1.0 {
            continue;
        }

        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    return random_in_unit_sphere().normalize();
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}
