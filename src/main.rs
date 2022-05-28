fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = dot(&oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let descriminant = half_b * half_b - a*c;

    if descriminant < 0.0 {
        -1.0
    } else {
        (-half_b - descriminant.sqrt()) / a
    }
}


fn ray_color<'a>(r: &'a Ray) -> Color {
    let t = hit_sphere(&Vec3(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = unit_vector(&(r.at(t) - Vec3(0.0, 0.0, -1.0)));
        return 0.5 * Vec3(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    
    // Image display config
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    // Camera config
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical/ 2.0 - Vec3(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);
            let direction = lower_left_corner + (u * horizontal) + (v * vertical) - origin;
            let r = Ray(&origin, &direction);
            let pixel_color: Color = ray_color(&r);
            write_color(&pixel_color);
        }
    }

    eprintln!("\nDone. ");
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3(f64, f64, f64);


impl Vec3 {

    fn x(&self) -> f64 {

        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
    }
}


use std::fmt;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

use std::ops::{Add, Sub, Mul, Div, Neg, DivAssign, MulAssign, AddAssign, SubAssign};

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


fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3(u.y() * v.z() - u.z() * v.y(),
         u.z() * v.x() - u.x() * v.z(),
         u.x() * v.y() - u.y() * v.x())
}

fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

type Color = Vec3;
type Point3 = Vec3;

fn write_color(pixel_color: &Color) {
    println!("{} {} {}",
        (255.999 * pixel_color.x()) as i64,
        (255.999 * pixel_color.y()) as i64,
        (255.999 * pixel_color.z()) as i64);
}


#[derive(Debug)]
struct Ray<'a>(&'a Point3, &'a Vec3);

impl Ray<'_> {
    fn origin(&self) -> &Point3 {
        self.0
    }

    fn direction(&self) -> &Vec3 {
        self.1
    }

    fn at(&self, t: f64) -> Point3 {
        *self.0 + t*(*self.1)
    }
}


//
#[derive(Debug, Copy, Clone)]
struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal= if self.front_face {*outward_normal} else {-*outward_normal};
    }
}

trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Debug, Copy, Clone)]
struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let descriminant = half_b * half_b - a*c;

        if descriminant < 0.0 {
            return false;
        }

        let sqrd = descriminant.sqrt();

        // nearest root within range
        let mut root = (-half_b - sqrd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        return true;
    }
}

// struct HittableList<'a> {
//     objects: Vec<&'a dyn Hittable>,
// }

// impl HittableList<'_> {
//     fn clear(&mut self) {
//         self.objects.drain(..);
//         self.objects.shrink_to(0);
//     }

//     fn add(&mut self, object: &'_ dyn Hittable) {
//         self.objects.push(object);
//     }

//     fn new(object: &dyn Hittable) -> Self {
//         let hittable_list = Self{objects: Vec::new()};
//         hittable_list.add(object);
//         hittable_list
//     }
// }

// impl Hittable for HittableList<'_> {
//     fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
//         true
//     }
// }