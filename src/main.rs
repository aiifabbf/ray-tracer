mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

fn color(ray: &Ray) -> Vec3 {
    let t = hitSphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = (ray.pointAtParameter(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
        return Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
    } else {
        let unitDirection = ray.direction().normalized();
        let t = 0.5 * unitDirection.y() + 1.0;
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn hitSphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = *ray.origin() - *center;
    let a = ray.direction().dot(ray.direction());
    let b = oc.dot(ray.direction()) * 2.0;
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (- b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn main() {
    let width = 1000;
    let height = 500;
    println!("P3");
    println!("{:?} {:?}", width, height);
    println!("255");

    let lowerLeft = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for y in (0..height).rev() {
        for x in 0..width {
            let u = x as f64 / width as f64;
            let v = y as f64 / height as f64;
            let ray = Ray::new(origin, lowerLeft + horizontal * u + vertical * v);
            let pixel = color(&ray);
            println!(
                "{:?} {:?} {:?}",
                (pixel.r() * 256.0) as usize,
                (pixel.g() * 256.0) as usize,
                (pixel.b() * 256.0) as usize,
            );
        }
    }
}
