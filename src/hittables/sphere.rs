use crate::hit::{HitRecord, Hitable};
use crate::vec3::{Point3};
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(center:Point3, radius:f64) -> Self {
        return Self{center:center, radius:radius}
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray:&Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                let mut rec = HitRecord::new( p, normal, t );
                rec.set_face_normal(&ray, normal);
                return Some(rec)
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                let mut rec = HitRecord::new( p, normal, t );
                rec.set_face_normal(&ray, normal);
                return Some(rec)
            }
        }
        None
    }
}