// 这个竟然要60 min……

extern crate ray_tracer;

use ray_tracer::camera::Camera;
use ray_tracer::camera::PerspectiveCamera;
use ray_tracer::geometry::Cube;
use ray_tracer::geometry::Rectangle;
use ray_tracer::mat4::Mat4;
use ray_tracer::material::DiffuseLight;
use ray_tracer::material::Lambertian;
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
    let width = 800;
    let height = 800;
    println!("P3");
    println!("{:?} {:?}", width, height);
    println!("255");

    let redMaterial = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let whiteMaterial = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let greenMaterial = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let lightMaterial = Arc::new(DiffuseLight::new(Vec3::new(15.0, 15.0, 15.0)));

    let greenWall = Sprite::builder()
        .geometry(Rectangle::new(555.0, 555.0).into())
        .material(greenMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(555.0, 555.0 / 2.0, 555.0 / 2.0))
                .multiplied(&Mat4::rotation((-90.0 as f64).to_radians(), Vec3::ey())),
        )
        .build();
    let redWall = Sprite::builder()
        .geometry(Rectangle::new(555.0, 555.0).into())
        .material(redMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(0.0, 555.0 / 2.0, 555.0 / 2.0))
                .multiplied(&Mat4::rotation((90.0 as f64).to_radians(), Vec3::ey())),
        )
        .build();
    let light = Sprite::builder()
        .geometry(Rectangle::new(130.0, 105.0).into())
        .material(lightMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(555.0 / 2.0, 554.0, 555.0 / 2.0))
                .multiplied(&Mat4::rotation((90.0 as f64).to_radians(), Vec3::ex())),
        )
        .build();
    let floor = Sprite::builder()
        .geometry(Rectangle::new(555.0, 555.0).into())
        .material(whiteMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(555.0 / 2.0, 0.0, 555.0 / 2.0))
                .multiplied(&Mat4::rotation((-90.0 as f64).to_radians(), Vec3::ex())),
        )
        .build();
    let ceiling = Sprite::builder()
        .geometry(Rectangle::new(555.0, 555.0).into())
        .material(whiteMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(555.0 / 2.0, 555.0, 555.0 / 2.0))
                .multiplied(&Mat4::rotation((90.0 as f64).to_radians(), Vec3::ex())),
        )
        .build();
    let backWall = Sprite::builder()
        .geometry(Rectangle::new(555.0, 556.0).into())
        .material(whiteMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(555.0 / 2.0, 555.0 / 2.0, 555.0))
                .multiplied(&Mat4::rotation((180.0 as f64).to_radians(), Vec3::ey())),
        )
        .build();

    let frontCube = Sprite::builder()
        .geometry(
            BoundingVolumeHierarchyNode::new(
                Cube::new(165.0, 165.0, 165.0)
                    .into_iter()
                    .map(|v| Arc::new(v) as Arc<dyn Bound<AxisAlignedBoundingBox>>)
                    .collect(),
            )
            .unwrap()
            .into(),
        )
        .material(whiteMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(212.5, 82.5, 147.5))
                .multiplied(&Mat4::rotation((-18.0 as f64).to_radians(), Vec3::ey())),
        )
        .build();
    // let frontCube = Sprite::builder()
    //     .geometry(Sphere::new(165.0 / 2.0).into())
    //     .material(Arc::new(Dielectric::new(1.5)))
    //     .transform(Mat4::translation(Vec3::new(212.5, 82.5, 147.5)))
    //     .build(); // 也可以替换成玻璃球
    let backCube = Sprite::builder()
        .geometry(
            BoundingVolumeHierarchyNode::new(
                Cube::new(165.0, 330.0, 165.0)
                    .into_iter()
                    .map(|v| Arc::new(v) as Arc<dyn Bound<AxisAlignedBoundingBox>>)
                    .collect(),
            )
            .unwrap()
            .into(),
        )
        .material(whiteMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(347.5, 165.0, 377.5))
                .multiplied(&Mat4::rotation((15.0 as f64).to_radians(), Vec3::ey())),
        )
        .build();

    let world: Vec<Arc<dyn Bound<AxisAlignedBoundingBox>>> = vec![
        Arc::new(greenWall),
        Arc::new(redWall),
        Arc::new(light),
        Arc::new(floor),
        Arc::new(ceiling),
        Arc::new(backWall),
        Arc::new(frontCube),
        Arc::new(backCube),
    ];
    let world = BoundingVolumeHierarchyNode::new(world).unwrap();
    let world = Arc::new(world);

    let eye = Vec3::new(555.0 / 2.0, 555.0 / 2.0, -800.0);
    let center = Vec3::new(555.0 / 2.0, 555.0 / 2.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let camera = PerspectiveCamera::new(
        eye,
        center,
        up,
        (40.0 as f64).to_radians(),
        width as f64 / height as f64,
        10.0,
        0.0,
    );
    let camera = Arc::new(camera);

    let subPixelSampleCount = 1000; // 每个pixel细分成多少个sub pixel

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
                (pixel.b().sqrt() * 255.0).min(255.0) as usize, // 有时候会发现有的像素的rgb值超过了255
            );
        }
    }
}
