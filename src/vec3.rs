use std::ops::{Add,Sub, Mul, Div};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e:[f64;3]
}

impl Vec3 {
    pub fn x(&self)->f64{return self.e[0]}
    pub fn y(&self)->f64{return self.e[1]}
    pub fn z(&self)->f64{return self.e[2]}

    pub fn length(&self) -> f64 {
        let length_sqared = self.length_sqared();
        return length_sqared.sqrt()
    }

    pub fn new(x:f64, y:f64, z:f64)-> Vec3 {
        return Vec3{e:[x,y,z]}
    }

    pub fn cross(&self, v:Vec3)-> Vec3 {
        let x = self.e[1] * v.e[2] - v.e[2] * v.e[1];
        let y = self.e[2] * v.e[0] - v.e[0] * v.e[2];
        let z = self.e[0] * v.e[1] - v.e[1] * v.e[0];
        return Vec3::new(x, y, z)
    }

    pub fn length_sqared(&self)->f64 {
        return self.e[0].powi(2) + self.e[1].powi(2)+self.e[2].powi(2);
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.e[0] + other.e[0];
        let y = self.e[1] + other.e[1];
        let z = self.e[2] + other.e[2];
        return Self{e:[x,y,z]}
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.e[0] - other.e[0];
        let y = self.e[1] - other.e[1];
        let z = self.e[2] - other.e[2];        
        return Self{e:[x,y,z]}
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs:Self)->Self {
        let x = self.e[0] * rhs.e[0];
        let y = self.e[1] * rhs.e[1];
        let z = self.e[2] * rhs.e[2]; 
        return Self{e:[x,y,z]}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs:f64)->Self {
        let x = self.e[0] * rhs;
        let y = self.e[1] * rhs;
        let z = self.e[2] * rhs; 
        return Self{e:[x,y,z]}
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs:f64)->Self{
        if rhs == 0.0 {
            panic!("Div by 0 not allowed");
        }

        return self*(1.0/rhs)
    }
}

pub type Point3 = Vec3;
pub type Colour = Vec3;
