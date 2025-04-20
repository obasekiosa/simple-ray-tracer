mod color;
mod hit;
mod point;
mod ray;
mod sphere;
mod vec;

use crate::{
    vec::{Vec3, dot, unit_vector},
    point::Point3,
    ray::Ray,
    color::{Color, write_color}
};


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

