mod camera;
mod geometry;
mod material;
mod optimize;
mod ray;
mod render;
mod sprite;
mod util;
mod vec3;

use crate::camera::Camera; // 非要把trait也import进来才能调用trait方法
use crate::camera::PerspectiveCamera;
use crate::geometry::Sphere;
use crate::material::CheckerTexture;
use crate::material::Dielectric;
use crate::material::DiffuseLight;
use crate::material::ImageTexture;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::material::Texture;
use crate::ray::Hit;
use crate::render::color;
use crate::sprite::Sprite;
use crate::vec3::Vec3;

use crate::optimize::AxisAlignedBoundingBox;
use crate::optimize::Bound;
use crate::optimize::BoundingVolumeHierarchyNode;

use rand::thread_rng;
use rand::Rng; // generator.gen_range()居然会用到这个，匪夷所思

// 知道了，因为gen_range是trait方法，所以必须要引入trait才行

use std::sync::Arc;

fn main() {
    let width = 1600;
    let height = 800;
    println!("P3");
    println!("{:?} {:?}", width, height);
    println!("255");

    let image = Arc::new(image::open("./earthmap.jpg").unwrap());
    let mapping = move |uv: &(f64, f64)| -> Vec3 {
        let image = image.clone();
        let buffer = image.as_rgb8().unwrap();
        let (u, v) = uv;
        let pixel = buffer.get_pixel(
            (u * buffer.width() as f64) as u32,
            (v * buffer.height() as f64) as u32,
        );
        return Vec3::new(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
        );
    };
    let texture: Arc<dyn Texture> = Arc::new(ImageTexture::new(mapping));

    let world: Vec<Box<dyn Hit>> = vec![
        Box::new(Sprite::new(
            Some(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0))),
            Some(Arc::new(Lambertian::new(texture.clone()))),
        )),
        Box::new(Sprite::new(
            Some(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0))),
            Some(Arc::new(Lambertian::new(texture.clone()))),
        )),
        Box::new(Sprite::new(
            Some(Arc::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0))),
            Some(Arc::new(DiffuseLight::new(Vec3::new(4.0, 4.0, 4.0)))),
        )),
        // Box::new(Sprite::new(
        //     Some(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2000.0))),
        //     Some(Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0)))),
        // )), // 天空球
    ];
    let world = Arc::new(world);
    // let world = Arc::new(randomScene());

    let eye = Vec3::new(0.0, 2.0, 8.0);
    let center = Vec3::new(0.0, 2.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let camera = PerspectiveCamera::new(
        eye,
        center,
        up,
        (60.0 as f64).to_radians(),
        width as f64 / height as f64,
        15.0,
        0.0,
    );
    let camera = Arc::new(camera);

    // 黑色背景下噪点很多，不知道是什么问题
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
                    // 这个不太符合OpenGL的normalized device coordinate，啥时候改一下
                    // let ray = Ray::new(origin, lowerLeft + horizontal * u + vertical * v);
                    let ray = camera.ray(u, v);
                    // 书上这里把world变成了一个什么hit_list，我想不如直接给Vec<Box<dyn Hit>>实现Hit trait，这样多个物体和一个物体都满足Hit trait
                    pixel += color(&ray, world.as_ref(), 100); // 天哪&*是什么玩意，还是用as_ref()吧
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

fn randomScene() -> BoundingVolumeHierarchyNode<AxisAlignedBoundingBox> {
    let mut scene: Vec<Arc<dyn Bound<AxisAlignedBoundingBox>>> = vec![Arc::new(Sprite::new(
        Some(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0))),
        Some(Arc::new(Metal::new(Vec3::new(0.5, 0.5, 0.5), 0.5))),
    ))];

    let mut generator = thread_rng();

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
                    let albedo = if whichMaterial < 0.10 {
                        Vec3::new(0.0, 0.0, 0.0)
                    } else {
                        Vec3::new(1.0, 1.0, 1.0)
                    };

                    scene.push(Arc::new(Sprite::new(
                        Some(Arc::new(Sphere::new(center, 0.2))),
                        Some(Arc::new(Lambertian::new(albedo))),
                    )));
                } else if whichMaterial < 0.6 {
                    let albedo = Vec3::new(1.0, 0.0, 0.0);
                    let fuzziness = generator.gen_range(0.0, 0.5);

                    scene.push(Arc::new(Sprite::new(
                        Some(Arc::new(Sphere::new(center, 0.2))),
                        Some(Arc::new(Metal::new(albedo, fuzziness))),
                    )));
                } else {
                    scene.push(Arc::new(Sprite::new(
                        Some(Arc::new(Sphere::new(center, 0.2))),
                        Some(Arc::new(Dielectric::new(1.5))),
                    )));
                }
            }
        }
    }

    let image = Arc::new(image::open("./earthmap.jpg").unwrap());
    let mapping = move |uv: &(f64, f64)| -> Vec3 {
        let image = image.clone();
        let buffer = image.as_bgr8().unwrap();
        let (u, v) = uv;
        let pixel = buffer.get_pixel(
            (u * buffer.width() as f64) as u32,
            ((1.0 - v) * buffer.height() as f64) as u32,
        );
        return Vec3::new(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
        );
    };

    let texture: Arc<dyn Texture> = Arc::new(ImageTexture::new(mapping));

    scene.extend(
        vec![
            Arc::new(Sprite::new(
                Some(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0))),
                Some(Arc::new(Lambertian::new(texture))),
            )) as Arc<dyn Bound<AxisAlignedBoundingBox>>,
            Arc::new(Sprite::new(
                Some(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0))),
                Some(Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))),
            )),
            Arc::new(Sprite::new(
                Some(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0))),
                Some(Arc::new(Dielectric::new(1.5))),
            )),
        ]
        .into_iter(),
    );

    for v in scene.iter() {
        eprintln!("{:#?}", v.bound());
    }

    return BoundingVolumeHierarchyNode::new(scene).unwrap();
}
