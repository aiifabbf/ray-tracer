mod geometry;
mod ray;
mod vec3;

use geometry::Sphere;
use ray::Hit;
use ray::Ray;
use vec3::Vec3;

fn color(ray: &Ray, world: &dyn Hit) -> Vec3 {
    let record = world.hit(ray);
    if record.is_some() {
        let record = record.unwrap();
        let t = record.t();
        let normal = (ray.pointAtParameter(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
        return Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
    } else {
        // 背景
        let unitDirection = ray.direction().normalized();
        let t = 0.5 * unitDirection.y() + 1.0;
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
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

    let mut world: Vec<Box<dyn Hit>> = vec![];

    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    for y in (0..height).rev() {
        for x in 0..width {
            let u = x as f64 / width as f64;
            let v = y as f64 / height as f64;
            let ray = Ray::new(origin, lowerLeft + horizontal * u + vertical * v);
            let pixel = color(&ray, &world);
            println!(
                "{:?} {:?} {:?}",
                (pixel.r() * 256.0) as usize,
                (pixel.g() * 256.0) as usize,
                (pixel.b() * 256.0) as usize,
            );
        }
    }
}
