
use super::hit::{Hittable, HitRecord};
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::utils::dot;

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere{
    pub fn new(center:Point3, radius:f64) -> Sphere {
        return Sphere{center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray:Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        let oc:Vec3 = ray.origin() - self.center;
        let a: f64 = ray.direction().length_sqared();
        let half_b = dot(oc, ray.direction());
        let c: f64 = oc.length_sqared() - self.radius.powi(2);
        
        let discriminatnt = half_b.powi(2) - a*c;
        if discriminatnt < 0.0 {return None};

        let mut root:f64 = (-half_b - discriminatnt.sqrt()) / a;

        if root < t_min || t_max < root {
            root = (-half_b + discriminatnt.sqrt()) / a;
            if root < t_min || t_max < root {
                return None
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: ray.at(root),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };
    
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&ray, outward_normal);

        return Some(rec)
    }
}