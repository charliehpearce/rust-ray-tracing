use crate::hittables::lib::HitRecord;
use crate::ray::Ray;
use crate::vec3::{near_zero, random_in_unit_sphere, random_unit_vector, reflect, Colour};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Colour)>;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Lambertian {
        return Lambertian { albedo: albedo };
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Colour)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if near_zero(
            scatter_direction.x,
            scatter_direction.y,
            scatter_direction.z,
        ) {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.p, scatter_direction);

        return Some((scattered, self.albedo));
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Metal {
        return Metal {
            albedo: albedo,
            fuzz: fuzz,
        };
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Colour)> {
        let reflected = reflect(ray_in.direction().normalize(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * random_in_unit_sphere(),
        );

        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            return Some((scattered, self.albedo));
        }

        None
    }
}
