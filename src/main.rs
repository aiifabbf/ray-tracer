mod geometry;
mod material;
mod ray;
mod sprite;
mod util;
mod vec3;

use geometry::Sphere;
use material::Lambertian;
use material::Metal;
use ray::Hit;
use ray::Ray;
use sprite::Sprite;
use util::randomInUnitSphere;
use vec3::Vec3;

use rand::thread_rng;
use rand::Rng; // generator.gen_range()居然会用到这个，匪夷所思

use std::sync::Arc;

fn color(ray: &Ray, world: &dyn Hit) -> Vec3 {
    if let Some(record) = world.hit(ray) {
        if let Some(material) = record.material() {
            if let Some((scattered, attenuation)) = material.scatter(ray, &record) {
                return attenuation * color(&scattered, world);
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
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

    let mut world: Vec<Box<dyn Hit + Send + Sync>> = vec![
        Box::new(Sprite::new(
            Some(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5))),
            Some(Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))),
        )),
        Box::new(Sprite::new(
            Some(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))),
            Some(Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))),
        )),
        Box::new(Sprite::new(
            Some(Arc::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5))),
            Some(Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)))),
        )),
        Box::new(Sprite::new(
            Some(Arc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5))),
            Some(Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)))),
        )),
    ];

    let world = std::sync::Arc::new(world);

    let subPixelSampleCount = 16; // 每个pixel细分成多少个sub pixel
    let (sender, receiver) = std::sync::mpsc::channel();
    let mut buffer = vec![vec![Vec3::new(0.0, 0.0, 0.0); width]; height];

    let executor = threadpool::ThreadPool::new(num_cpus::get());

    for y in (0..height).rev() {
        for x in 0..width {
            let sender = sender.clone();
            let world = world.clone();

            executor.execute(move || {
                let mut pixel = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..subPixelSampleCount {
                    let mut generator = thread_rng();
                    let u = (x as f64 + generator.gen_range(0.0, 1.0)) / width as f64;
                    let v = (y as f64 + generator.gen_range(0.0, 1.0)) / height as f64;
                    let ray = Ray::new(origin, lowerLeft + horizontal * u + vertical * v);
                    pixel += color(&ray, world.as_ref());
                }

                pixel /= subPixelSampleCount as f64;
                sender.send((x, y, pixel));
            });
        }
    }

    for _ in (0..height).rev() {
        for _ in 0..width {
            let (x, y, pixel) = receiver.recv().unwrap();
            buffer[y][x] = pixel;
        }
    }

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = &buffer[y][x];
            println!(
                "{:?} {:?} {:?}",
                (pixel.r().sqrt() * 255.0) as usize,
                (pixel.g().sqrt() * 255.0) as usize,
                (pixel.b().sqrt() * 255.0) as usize,
            );
        }
    }
}
