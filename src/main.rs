mod camera;
mod geometry;
mod mat4;
mod material;
mod optimize;
mod ray;
mod render;
mod sprite;
mod util;
mod vec3;
mod vec4;

use crate::camera::Camera; // 非要把trait也import进来才能调用trait方法
use crate::camera::PerspectiveCamera;
use crate::geometry::Rectangle;
use crate::geometry::Sphere;
use crate::mat4::Mat4;
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
    let width = 800;
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
            ((1.0 - v) * buffer.height() as f64) as u32,
        ); // image的原点是图像的左上角，而uv坐标系里原点是左下角，所以这里要颠倒一下
        return Vec3::new(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
        );
    };
    // let texture: Arc<dyn Texture> = Arc::new(ImageTexture::new(mapping));

    let redMaterial = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let whiteMaterial = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let greenMaterial = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let lightMaterial = Arc::new(DiffuseLight::new(Vec3::new(15.0, 15.0, 15.0)));

    let greenWall = Sprite::builder()
        .geometry(Rectangle::new(555.0, 555.0))
        // .geometry(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 555.0 / 2.0))
        .material(greenMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(555.0, 555.0 / 2.0, 555.0 / 2.0))
                .multiplied(&Mat4::rotation((-90.0 as f64).to_radians(), Vec3::ey())),
        )
        .build();
    let redWall = Sprite::builder()
        .geometry(Rectangle::new(555.0, 555.0))
        .material(redMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(0.0, 555.0 / 2.0, 555.0 / 2.0))
                .multiplied(&Mat4::rotation((90.0 as f64).to_radians(), Vec3::ey())),
        )
        .build();
    let light = Sprite::builder()
        .geometry(Rectangle::new(130.0, 105.0))
        .material(lightMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(555.0 / 2.0, 554.0, 555.0 / 2.0))
                .multiplied(&Mat4::rotation((90.0 as f64).to_radians(), Vec3::ex())),
        )
        .build();
    let floor = Sprite::builder()
        .geometry(Rectangle::new(555.0, 555.0))
        .material(whiteMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(555.0 / 2.0, 0.0, 550.0 / 2.0))
                .multiplied(&Mat4::rotation((-90.0 as f64).to_radians(), Vec3::ex())),
        )
        .build();
    let ceiling = Sprite::builder()
        .geometry(Rectangle::new(555.0, 555.0))
        .material(whiteMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(555.0 / 2.0, 555.0, 550.0 / 2.0))
                .multiplied(&Mat4::rotation((90.0 as f64).to_radians(), Vec3::ex())),
        )
        .build();
    let backWall = Sprite::builder()
        .geometry(Rectangle::new(555.0, 555.0))
        .material(whiteMaterial.clone())
        .transform(
            Mat4::translation(Vec3::new(555.0 / 2.0, 555.0 / 2.0, 555.0))
                .multiplied(&Mat4::rotation((180.0 as f64).to_radians(), Vec3::ey())),
        )
        .build();

    let world: Vec<Box<dyn Hit>> = vec![
        Box::new(greenWall),
        Box::new(redWall),
        Box::new(light),
        Box::new(floor),
        Box::new(ceiling),
        Box::new(backWall),
    ];
    let world = Arc::new(world);
    // let world = Arc::new(randomScene());

    let eye = Vec3::new(555.0 / 2.0, 550.0 / 2.0, -800.0);
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

    // 黑色背景下噪点很多，不知道是什么问题
    let subPixelSampleCount = 256; // 每个pixel细分成多少个sub pixel

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
            // eprintln!("{:#?} {:#?} {:#?}", y, x, pixel);
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
