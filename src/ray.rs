pub use crate::vec3::{Vec3, Point3, Colour};
pub use crate::utils::{dot, unit_vector};


#[derive(Clone, Copy)]
pub struct Ray{
    origin:Point3,
    dir:Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        return Ray{origin:origin, dir:direction}

    }

    pub fn origin(&self) -> Point3 {
        return self.origin
    }

    pub fn direction(&self) -> Vec3 {
        return self.dir
    }

    pub fn at(&self, t:f64) -> Point3 {
        return self.origin + self.dir*t
    }

    pub fn ray_colour(&self)->Colour{
        let t:f64 = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, *self); 
        
        if t > 0.0  {
            let n:Vec3 = unit_vector(self.at(t) - Vec3::new(0.0,0.0,-1.0));
            return Colour::new(n.x()+1.0,n.y()+1.0,n.z()+1.0)*0.5
        }
            let unit_direction = unit_vector(self.direction());
            let t = 0.5*(unit_direction.y()+1.0);
            return Colour::new(1.0, 1.0, 1.0)*(1.0-t) + Colour::new(0.5, 0.7, 1.0)*t
    }
}