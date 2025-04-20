use std::{
    fmt, 
    ops::{
        Add,
        Sub,
        Mul,
        Div,
        Neg,
        DivAssign,
        MulAssign,
        AddAssign,
        SubAssign
    }
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);


impl Vec3 {

    pub fn x(&self) -> f64 {

        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        dot(self, other)
    }

    pub fn cross(&self, other: &Self) -> Self {
        cross(self, other)
    }

    pub fn unit(&self) -> Self {
        unit_vector(self)
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3(u.y() * v.z() - u.z() * v.y(),
         u.z() * v.x() - u.x() * v.z(),
         u.x() * v.y() - u.y() * v.x())
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl SubAssign for Vec3 {

    fn sub_assign(&mut self, v: Self) {
        *self = Self(self.0 - v.0, self.1 - v.1, self.2 - v.2);
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Self) {
        *self = Self(self.0 + v.0, self.1 + v.1, self.2 + v.2);
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self * 1.0/t;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Self(self.0 * t, self.1 * t, self.2 * t);
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3(self.0 - v.0, self.1 - v.1, self.2 - v.2)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3(self.0 * v.0, self.1 * v.1, self.2 * v.2)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}


impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }
}

