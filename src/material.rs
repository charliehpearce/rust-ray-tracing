use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::vec3::Colour;

pub trait Material {
    fn scatter(ray_in:&Ray, hit_record:HitRecord, attenutation:Colour, scattered_ray:&Ray) -> bool;
}