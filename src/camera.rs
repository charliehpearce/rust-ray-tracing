use crate::ray::Ray;
use crate::vec3::{random_in_unit_disk, Point3, Vec3};
use std::f64::consts::PI;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

fn degrees_to_radians(degrees: f64) -> f64 {
    return (degrees * PI) / 180.0;
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let h = f64::tan(degrees_to_radians(vfov) / 2.0);
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w: Vec3 = (lookfrom - lookat).normalize();
        let u: Vec3 = vup.cross(&w).normalize();
        let v: Vec3 = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        let lens_radius = aperture / 2.0;

        return Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
        };
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk() * 0.1;
        let offset = self.u * rd.x + self.v * rd.y;

        return Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        );
    }
}
