use std::ops::Range;

pub use nalgebra::Vector3;
use rand::{thread_rng, Rng};

pub type Vec3 = Vector3<f64>;
pub type Point3 = Vec3;
pub type Colour = Vec3;

fn random_vec3() -> Vec3 {
    let mut rng = thread_rng();
    return Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))
}

fn random_rng_vec3(range:Range<f64>) -> Vec3 {
    let mut rng = thread_rng();
    return Vec3::new(rng.gen_range(range.clone()), rng.gen_range(range.clone()), rng.gen_range(range.clone()))
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_rng_vec3(-1.0..1.0);
        
        if p.dot(&p) >= 1.0 {
            continue;
        }
        
        return p
    }
}