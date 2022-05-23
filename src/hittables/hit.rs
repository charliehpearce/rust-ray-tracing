use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::utils::dot;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord{
    // pub fn new(p:Point3, t:f64 , normal:Vec3, front_face:bool = false) -> HitRecord {
    //     return HitRecord{p:p, normal:normal, t:t, front_face:front_face}
    // }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        self.front_face = dot(r.direction(),outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * (-1.0)
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray:Ray, t_min:f64, t_max:f64) -> Option<HitRecord>;
}


pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        return HittableList { objects: Vec::new() }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl 'static + Hittable) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray:Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        let mut hit_anything:Option<HitRecord> = None;
        let mut closest_so_far:f64 = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                hit_anything = Some(hit);
                closest_so_far = hit.t;
            }
        }
        
        return hit_anything
    }
}
