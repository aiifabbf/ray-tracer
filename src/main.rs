mod geometry;
mod ray;
mod vec3;

use geometry::Sphere;
use ray::Hit;
use ray::Ray;
use vec3::Vec3;

use rand::random;
use rand::thread_rng;
use rand::Rng;

use std::f64;

fn randomInUnitSphere() -> Vec3 {
    let mut p = Vec3::new(1.0, 1.0, 1.0);

    while p.dot(&p) >= 1.0 {
        p = Vec3::new(random::<f64>(), random::<f64>(), random::<f64>()) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);
    }

    return p;
}
// 书上用的是这个奇怪的球面向量生成器，但是我感觉这不就是一个很简单的变换吗……

// fn randomInUnitSphere() -> Vec3 {
//     let theta = random::<f64>().fract();
//     let phi = random::<f64>().fract();
//     let r = random::<f64>().fract();

//     return Vec3::new(r * phi.sin() * theta.cos(), r * phi.sin() * theta.sin(), r * phi.cos());
// }
// 试了下，果然不是很均匀……

fn color(ray: &Ray, world: &dyn Hit) -> Vec3 {
    let record = world.hit(ray);
    if record.is_some() {
        let record = record.unwrap();
        // let target = *record.intersection() + *record.normal() + Vec3::new(random::<f64>(), random::<f64>(), random::<f64>()).normalized();
        let target = *record.intersection() + *record.normal() + randomInUnitSphere().normalized();
        return color(
            &Ray::new(*record.intersection(), target - *record.intersection()),
            world,
        ) * 0.5;
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

    let mut world: Vec<Box<dyn Hit + Send + Sync>> = vec![];

    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let world = std::sync::Arc::new(world);

    
    let subPixelSampleCount = 16; // 每个pixel细分成多少个sub pixel
    
    let (sender, receiver) = std::sync::mpsc::channel();
    let mut buffer = vec![vec![Vec3::new(0.0, 0.0, 0.0); width]; height];
    
    for y in (0..height).rev() {
        for x in 0..width {
            let sender = sender.clone();
            let world = world.clone();
            
            std::thread::spawn(move || {
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
