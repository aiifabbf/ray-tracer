// 这个例子在我i5-3317U上大概要跑10 min

extern crate ray_tracer; // 不加这行的话，编译没问题，但是RLS就没有类型提示了，很怪。然而rust-analyzer有提示

use ray_tracer::camera::Camera;
use ray_tracer::camera::PerspectiveCamera;
use ray_tracer::geometry::Sphere;
use ray_tracer::mat4::Mat4;
use ray_tracer::material::Dielectric;
use ray_tracer::material::DiffuseLight;
use ray_tracer::material::Lambertian;
use ray_tracer::material::Metal;
use ray_tracer::optimize::AxisAlignedBoundingBox;
use ray_tracer::optimize::Bound;
use ray_tracer::optimize::BoundingVolumeHierarchyNode;
use ray_tracer::render::color;
use ray_tracer::sprite::Sprite;
use ray_tracer::vec3::Vec3;

use rand::thread_rng;
use rand::Rng;

use std::sync::Arc;

fn main() {
    let width = 1600;
    let height = 800;
    println!("P3");
    println!("{:?} {:?}", width, height);
    println!("255");

    let world = BoundingVolumeHierarchyNode::new(randomScene()).unwrap();
    let world = Arc::new(world);

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

    let subPixelSampleCount = 100;

    let (sender, receiver) = std::sync::mpsc::channel();
    let mut buffer = vec![vec![Vec3::new(0.0, 0.0, 0.0); width]; height];
    let cpuCount = num_cpus::get();

    for i in 0..cpuCount {
        let sender = sender.clone();
        let world = world.clone();
        let camera = camera.clone();

        std::thread::spawn(move || {
            for y in (0..height).rev() {
                if y % cpuCount != i {
                    continue;
                }

                for x in 0..width {
                    let mut pixel = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..subPixelSampleCount {
                        let mut generator = thread_rng();
                        let u = (x as f64 + generator.gen_range(0.0, 1.0)) / width as f64;
                        let v = (y as f64 + generator.gen_range(0.0, 1.0)) / height as f64;
                        let ray = camera.ray(u, v);
                        pixel += color(&ray, world.as_ref(), 100);
                    }
                    pixel /= subPixelSampleCount as f64;
                    sender.send((x, y, pixel)).unwrap();
                }
            }
        });
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
                (pixel.r().sqrt() * 255.0).min(255.0) as usize,
                (pixel.g().sqrt() * 255.0).min(255.0) as usize,
                (pixel.b().sqrt() * 255.0).min(255.0) as usize,
            );
        }
    }
}

fn randomScene() -> Vec<Arc<dyn Bound<AxisAlignedBoundingBox>>> {
    let mut scene: Vec<Arc<dyn Bound<AxisAlignedBoundingBox>>> = vec![
        Arc::new(
            Sprite::builder()
                .geometry(Sphere::new(1000.0).into())
                .material(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)).into())
                .transform(Mat4::translation(Vec3::new(0.0, -1000.0, 0.0)))
                .build(), // 地面实际上是个巨大的球
        ),
        Arc::new(
            Sprite::builder()
                .geometry(Sphere::new(2000.0).into())
                .material(DiffuseLight::new(Vec3::new(0.5, 0.7, 1.0)).into())
                .build(), // 天空球
        ),
    ];

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

                    scene.push(Arc::new(
                        Sprite::builder()
                            .geometry(Sphere::new(0.2).into())
                            .material(Lambertian::new(albedo).into())
                            .transform(Mat4::translation(center))
                            .build(),
                    ));
                } else if whichMaterial < 0.6 {
                    let albedo = Vec3::new(
                        generator.gen_range(0.5, 1.0),
                        generator.gen_range(0.5, 1.0),
                        generator.gen_range(0.5, 1.0),
                    );
                    let fuzziness = generator.gen_range(0.0, 0.5);

                    scene.push(Arc::new(
                        Sprite::builder()
                            .geometry(Sphere::new(0.2).into())
                            .material(Metal::new(albedo, fuzziness).into())
                            .transform(Mat4::translation(center))
                            .build(),
                    ));
                } else {
                    scene.push(Arc::new(
                        Sprite::builder()
                            .geometry(Sphere::new(0.2).into())
                            .material(Dielectric::new(1.5).into())
                            .transform(Mat4::translation(center))
                            .build(),
                    ));
                }
            }
        }
    }

    // 场景最中间的三个大球
    scene.extend(
        vec![
            Arc::new(
                Sprite::builder()
                    .geometry(Sphere::new(1.0).into())
                    .material(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)).into())
                    .transform(Mat4::translation(Vec3::new(-4.0, 1.0, 0.0)))
                    .build(),
            ) as Arc<dyn Bound<AxisAlignedBoundingBox>>,
            Arc::new(
                Sprite::builder()
                    .geometry(Sphere::new(1.0).into())
                    .material(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0).into())
                    .transform(Mat4::translation(Vec3::new(4.0, 1.0, 0.0)))
                    .build(),
            ),
            Arc::new(
                Sprite::builder()
                    .geometry(Sphere::new(1.0).into())
                    .material(Dielectric::new(1.5).into())
                    .transform(Mat4::translation(Vec3::new(0.0, 1.0, 0.0)))
                    .build(),
            ),
        ]
        .into_iter(),
    );

    return scene;
}
