mod camera;
mod geometry;
mod material;
mod ray;
mod sprite;
mod util;
mod vec3;

use camera::PerspectiveCamera;
use camera::Camera; // 非要把trait也import进来才能调用trait方法
use geometry::Sphere;
use material::Dielectric;
use material::Lambertian;
use material::Metal;
use ray::Hit;
use ray::Ray;
use sprite::Sprite;
use vec3::Vec3;

use rand::thread_rng;
use rand::Rng; // generator.gen_range()居然会用到这个，匪夷所思

use std::sync::Arc;

fn color(ray: &Ray, world: &dyn Hit, maxDepth: usize) -> Vec3 {
    if maxDepth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(record) = world.hit(ray) {
        if let Some(material) = record.material() {
            if let Some((scattered, attenuation)) = material.scatter(ray, &record) {
                return attenuation * color(&scattered, world, maxDepth - 1);
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        // 背景
        let unitDirection = ray.direction().normalized();
        let t = 0.5 * (unitDirection.y() + 1.0);
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn main() {
    let width = 1600;
    let height = 800;
    println!("P3");
    println!("{:?} {:?}", width, height);
    println!("255");

    let lowerLeft = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    // 这里如果把Hit声明为Send + Sync的子trait，就不会报错
    let mut world: Vec<Box<dyn Hit>> = vec![
        Box::new(Sprite::new(
            Some(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5))),
            Some(Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)))),
        )),
        Box::new(Sprite::new(
            Some(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))),
            Some(Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))),
        )),
        Box::new(Sprite::new(
            Some(Arc::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5))),
            Some(Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0))),
        )),
        Box::new(Sprite::new(
            Some(Arc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5))),
            Some(Arc::new(Dielectric::new(1.5))),
        )),
    ];
    let world = Arc::new(world);

    let camera = PerspectiveCamera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        (60.0 as f64).to_radians(),
        width as f64 / height as f64,
    );
    let camera = Arc::new(camera);

    // 书上这个设置的是100，但是我调成128都没法达到书上那个图那么少的噪点……
    let subPixelSampleCount = 100; // 每个pixel细分成多少个sub pixel

    let (sender, receiver) = std::sync::mpsc::channel();
    let mut buffer = vec![vec![Vec3::new(0.0, 0.0, 0.0); width]; height];
    let executor = threadpool::ThreadPool::new(num_cpus::get());

    for y in (0..height).rev() {
        for x in 0..width {
            let sender = sender.clone();
            let world = world.clone();
            let camera = camera.clone();

            executor.execute(move || {
                let mut pixel = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..subPixelSampleCount {
                    let mut generator = thread_rng();
                    let u = (x as f64 + generator.gen_range(0.0, 1.0)) / width as f64;
                    let v = (y as f64 + generator.gen_range(0.0, 1.0)) / height as f64;
                    // let ray = Ray::new(origin, lowerLeft + horizontal * u + vertical * v);
                    let ray = camera.ray(u, v);
                    pixel += color(&ray, world.as_ref(), 100);
                    // 书上这里把world变成了一个什么hit_list，我想不如直接给Vec<Box<dyn Hit>>实现Hit trait，这样多个物体和一个物体都满足Hit trait
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
