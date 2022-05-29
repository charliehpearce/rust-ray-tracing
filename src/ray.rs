use crate::hit::{Hitable,World};

use crate::vec3::{Vec3,Point3, Colour, random_in_unit_sphere};
pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin:Point3, direction:Vec3) -> Self {
        return Ray{origin:origin, direction:direction}
    }

    pub fn origin(&self) -> Point3 {
        return self.origin
    }

    pub fn direction(&self) -> Vec3 {
        return self.direction
    }

    pub fn at(&self, t:f64) -> Point3 {
        return self.origin + t*self.direction
    }
}

pub fn ray_color (ray:&Ray, world:&World, depth:i32) -> Colour {

    if depth <= 0 {
        return Colour::new(0.0,0.0,0.0)
    }

    if let Some(hit_record) = world.hit(&ray, 0.001, f64::INFINITY) {
        let target = hit_record.p() + hit_record.normal() + random_in_unit_sphere();
        let ray = Ray::new(hit_record.p(), target-hit_record.p());
        return 0.5* ray_color(&ray, world, depth-1)
    };
    
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0-t)*Colour::new(1.0,1.0,1.0) + t*Colour::new(0.5,0.7,1.0)

}