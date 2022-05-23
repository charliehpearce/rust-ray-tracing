use std::f64::consts::PI;
use crate::vec3::Vec3;

pub fn dot(v1:Vec3, v2:Vec3)-> f64{
    return v1.x()*v2.x() + v1.y()*v2.y() + v1.z()*v2.z()
}

pub fn unit_vector(v:Vec3) -> Vec3 {
    return v/v.length()
}

pub fn degrees_to_radians(degs:f64) -> f64 {
    degs * PI / (180.0)
}