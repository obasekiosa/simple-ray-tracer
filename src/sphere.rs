use crate::{
    hit::{HitRecord, Hittable},
    ray::Ray,
    point::Point3
};

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
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
