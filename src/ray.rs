use crate::hittables::lib::{Hitable, World};

use crate::vec3::{Colour, Point3, Vec3};
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        return Ray {
            origin: origin,
            direction: direction,
        };
    }

    pub fn origin(&self) -> Point3 {
        return self.origin;
    }

    pub fn direction(&self) -> Vec3 {
        return self.direction;
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t * self.direction;
    }
}

pub fn ray_colour(ray: &Ray, world: &World, depth: i32) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(&ray, 0.001, f64::INFINITY) {
        if let Some((scattered, attenutation)) = hit_record.material.scatter(ray, &hit_record) {
            return attenutation.zip_map(&ray_colour(&scattered, world, depth - 1), |r, s| r * s);
        }
    };

    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    return (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0);
}
