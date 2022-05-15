pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

pub trait Hittable {
    fn hit(&self, ray:Ray, t_min:f64, t_max:f64, rec:HitRecord) -> bool;
}

struct Sphere {
    center: Point3,
    raduis: f64
}

impl Sphere{
    pub fn new(center:Point3, raduis:f64) {
        return Sphere{center, raduis}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray:Ray, t_min:f64, t_max:f64, rec:HitRecord) -> bool {
        let oc:Vec3 = ray.origin() - self.center;
        let a: f64 = ray.direction().length_sqared();
        let half_b = dot(oc, ray.direction());
        let c: f64 = oc.length_sqared() - self.raduis.powi(2);
        
        let discriminatnt = half_b.powi(2) - a*c;
        if discriminatnt < 0.0 {return false};

        let root:f64 = -half_b - discriminatnt.sqrt() / a;

        if root < t_min || t_max < root {
            root = (-half_b + discriminatnt.sqrt()) / a;
            if root < t_min || t_max < root {
                return false
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.normal = (rec.p - self.center) / self.raduis;

        return true
    }
}