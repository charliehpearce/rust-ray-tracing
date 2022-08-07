use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: &'a dyn Material,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Point3, normal: Vec3, t: f64, matierial: &'a dyn Material) -> HitRecord {
        // assume ray always starts outside of hitable, so front face is true
        return HitRecord {
            p: p,
            normal: normal,
            t: t,
            material: matierial,
            front_face: true,
        };
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) -> bool {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.front_face = front_face;

        if front_face {
            self.normal = outward_normal
        } else {
            self.normal = -outward_normal
        }

        return front_face;
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitableList {
    objects: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        return HitableList {
            objects: Vec::new(),
        };
    }

    pub fn add<T: Hitable + 'static>(&mut self, object: T) -> () {
        self.objects.push(Box::new(object))
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if let Some(hit_record) = obj.hit(ray, t_min, closest_so_far) {
                hit_anything = Some(hit_record);
                closest_so_far = hit_record.t;
            };
        }

        return hit_anything;
    }
}

pub type World = HitableList;
