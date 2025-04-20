use crate::{
    vec::Vec3,
    point::Point3
};


#[derive(Debug)]
pub struct Ray<'a>(pub &'a Point3, pub &'a Vec3);

impl Ray<'_> {
    pub fn origin(&self) -> &Point3 {
        self.0
    }

    pub fn direction(&self) -> &Vec3 {
        self.1
    }

    pub fn at(&self, t: f64) -> Point3 {
        *self.0 + t*(*self.1)
    }
}