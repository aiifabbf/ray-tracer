extern crate ray_tracer; // 不加这行的话，编译没问题，但是RLS就没有类型提示了，很怪。然而rust-analyzer有提示

use ray_tracer::camera::Camera; // 非要把trait也import进来才能调用trait方法
use ray_tracer::camera::PerspectiveCamera;
use ray_tracer::geometry::Sphere;
use ray_tracer::material::Dielectric;
use ray_tracer::material::Lambertian;
use ray_tracer::material::Metal;
use ray_tracer::ray::Hit;
use ray_tracer::ray::Ray;
use ray_tracer::render::color;
use ray_tracer::sprite::Sprite;
use ray_tracer::vec3::Vec3;

use rand::thread_rng;
use rand::Rng; // generator.gen_range()居然会用到这个，匪夷所思

use std::sync::Arc;

fn main() {
    let width = 1600;
    let height = 800;
    println!("P3");
    println!("{:?} {:?}", width, height);
    println!("255");

    // 这里如果把Hit声明为Send + Sync的子trait，就不会报错
    let world = Arc::new(randomScene());

    let eye = Vec3::new(13.0, 2.0, 3.0);
    let center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let camera = PerspectiveCamera::new(
        eye,
        center,
        up,
        (20.0 as f64).to_radians(),
        width as f64 / height as f64,
        10.0,
        0.05,
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
                sender.send((x, y, pixel)).unwrap();
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

fn randomScene() -> Vec<Box<dyn Hit>> {
    let mut scene: Vec<Box<dyn Hit>> = vec![Box::new(Sprite::new(
        Some(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0))),
        Some(Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))),
    ))]; // 地面实际上是个巨大的球

    let mut generator = thread_rng();

    // 随机生成一些小球
    for a in -11..11 {
        for b in -11..11 {
            let whichMaterial = generator.gen_range(0.0, 1.0);
            let center = Vec3::new(
                a as f64 + 0.9 * generator.gen_range(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * generator.gen_range(0.0, 1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if whichMaterial < 0.3 {
                    let mut albedo = Vec3::new(
                        generator.gen_range(0.0, 1.0),
                        generator.gen_range(0.0, 1.0),
                        generator.gen_range(0.0, 1.0),
                    );
                    albedo = albedo * albedo;

                    scene.push(Box::new(Sprite::new(
                        Some(Arc::new(Sphere::new(center, 0.2))),
                        Some(Arc::new(Lambertian::new(albedo))),
                    )));
                } else if whichMaterial < 0.6 {
                    let albedo = Vec3::new(
                        generator.gen_range(0.5, 1.0),
                        generator.gen_range(0.5, 1.0),
                        generator.gen_range(0.5, 1.0),
                    );
                    let fuzziness = generator.gen_range(0.0, 0.5);

                    scene.push(Box::new(Sprite::new(
                        Some(Arc::new(Sphere::new(center, 0.2))),
                        Some(Arc::new(Metal::new(albedo, fuzziness))),
                    )));
                } else {
                    scene.push(Box::new(Sprite::new(
                        Some(Arc::new(Sphere::new(center, 0.2))),
                        Some(Arc::new(Dielectric::new(1.5))),
                    )));
                }
            }
        }
    }

    // 场景最中间的三个大球
    scene.extend(
        vec![
            Box::new(Sprite::new(
                Some(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0))),
                Some(Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))),
            )) as Box<dyn Hit>,
            Box::new(Sprite::new(
                Some(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0))),
                Some(Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))),
            )),
            Box::new(Sprite::new(
                Some(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0))),
                Some(Arc::new(Dielectric::new(1.5))),
            )),
        ]
        .into_iter(),
    );

    return scene;
}
