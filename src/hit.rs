use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::material::Material;

#[derive(Clone, Copy)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    // material:T
}

impl HitRecord {
    pub fn new(p:Point3, normal:Vec3, t:f64) -> HitRecord {
        return HitRecord{p:p, normal:normal, t:t}
    }

    // pub fn t(&self) -> f64 {
    //     return self.t
    // }

    pub fn normal(&self) -> Vec3 {
        return self.normal
    }

    pub fn p(&self) -> Point3 {
        return self.p
    }

    pub fn set_face_normal(&mut self, ray:&Ray, outward_normal:Vec3) -> bool {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        if !front_face {
            self.normal = -self.normal
        }
        return front_face
        
    }
}

pub trait Hitable {
    fn hit(&self, ray:&Ray, t_min:f64, t_max:f64) -> Option<HitRecord>;
}

pub struct HitableList {
    objects: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new() -> HitableList {
        return HitableList{objects:Vec::new()}
    }
    // pub fn clear(&mut self) -> (){
    //     self.objects.clear()
    // }

    pub fn add<T:Hitable + 'static>(&mut self, object:T) -> () {
        self.objects.push(Box::new(object))
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray:&Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if let Some(hit_record) = obj.hit(ray, t_min, closest_so_far) {
                hit_anything = Some(hit_record);
                closest_so_far = hit_record.t;
            };
        }

        return hit_anything

    }
}

pub type World = HitableList;
