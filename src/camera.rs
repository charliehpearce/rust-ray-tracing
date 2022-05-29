use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

pub struct Camera {
    origin : Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera{
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT:f64 = 2.0;
        let viewport_width = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENTH:f64 = 1.0;
    
        let origin = Point3::new(0.0,0.0,0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0,FOCAL_LENTH);

        return Camera{origin, lower_left_corner, horizontal, vertical}
    }

    pub fn get_ray(&self, u:f64, v:f64) -> Ray{
        return Ray::new(self.origin, self.lower_left_corner+ u*self.horizontal + v*self.vertical -self.origin)

    }
}
