use crate::{
    point::Point3,
    vec::{Vec3, dot},
    ray::Ray
};


#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    normal: Vec3,
    pub t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal= if self.front_face {*outward_normal} else {-*outward_normal};
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

struct HittableList<'a> {
    objects: Vec<&'a dyn Hittable>,
}

impl HittableList<'_> {
    fn clear(&mut self) {
        self.objects.drain(..);
        self.objects.shrink_to(0);
    }

    // fn add<'a, 'b>(&'b mut self, object: &'a dyn Hittable) {
    //     self.objects.push(object);
    // }

    // fn new(object: &dyn Hittable) -> Self {
    //     let mut hittable_list = Self{objects: Vec::new()};
    //     hittable_list.add(object);
    //     hittable_list
    // }
}

impl Hittable for HittableList<'_> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        true
    }
}