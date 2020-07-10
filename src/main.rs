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
mod volume;

use crate::camera::Camera; // 非要把trait也import进来才能调用trait方法
use crate::camera::PerspectiveCamera;
use crate::geometry::Cube;
use crate::geometry::Rectangle;
use crate::geometry::Sphere;
use crate::mat4::Mat4;
use crate::material::CheckerTexture;
use crate::material::Dielectric;
use crate::material::DiffuseLight;
use crate::material::ImageTexture;
use crate::material::Isotropic;
use crate::material::Lambertian;
use crate::material::Material;
use crate::material::Metal;
use crate::material::SolidColor;
use crate::material::Texture;
use crate::ray::Hit;
use crate::render::color;
use crate::sprite::Sprite;
use crate::vec3::Vec3;
use crate::volume::ConstantMedium;

use crate::optimize::AxisAlignedBoundingBox;
use crate::optimize::Bound;
use crate::optimize::BoundingVolumeHierarchyNode;

use rand::thread_rng;
use rand::Rng; // generator.gen_range()居然会用到这个，匪夷所思

use std::io::Write;

// 知道了，因为gen_range是trait方法，所以必须要引入trait才行

use std::sync::Arc;

fn main() {
    let width: usize = 800;
    let height: usize = 800;
    // println!("P3");
    // println!("{:?} {:?}", width, height);
    // println!("255");

    let world = BoundingVolumeHierarchyNode::new(finalScene()).unwrap();
    // eprintln!("{:#?}", world);
    let world = Arc::new(world);

    let eye = Vec3::new(555.0 / 2.0 + 200.0, 550.0 / 2.0, -600.0);
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
    let subPixelSampleCount = 1000; // 每个pixel细分成多少个sub pixel

    let (sender, receiver) = std::sync::mpsc::channel();
    // let mut buffer = vec![vec![Vec3::new(0.0, 0.0, 0.0); width]; height];
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
                        // 这个不太符合OpenGL的normalized device coordinate，啥时候改一下
                        // let ray = Ray::new(origin, lowerLeft + horizontal * u + vertical * v);
                        let ray = camera.ray(u, v);
                        // 书上这里把world变成了一个什么hit_list，我想不如直接给Vec<Box<dyn Hit>>实现Hit trait，这样多个物体和一个物体都满足Hit trait
                        pixel += color(&ray, world.as_ref(), 100); // 天哪&*是什么玩意，还是用as_ref()吧
                    }
                    pixel /= subPixelSampleCount as f64;
                    sender.send((x, y, pixel)).unwrap();
                }
            }
        });
    }

    // 改成了4个worker thread，这样可以减少Arc clone的次数

    // 改成输出png了，好像ppm很少有软件能打开
    // image库真难用啊……
    let mut img = image::DynamicImage::new_rgba8(width as u32, height as u32);

    for _ in (0..height).rev() {
        for _ in 0..width {
            let (x, y, pixel) = receiver.recv().unwrap();
            // buffer[y][x] = pixel;
            use image::GenericImage;
            img.put_pixel(
                x as u32,
                (height - 1 - y) as u32,
                image::Rgba([
                    (pixel.r().sqrt() * 255.0).min(255.0) as u8,
                    (pixel.g().sqrt() * 255.0).min(255.0) as u8,
                    (pixel.b().sqrt() * 255.0).min(255.0) as u8,
                    255,
                ]),
            );

            if x == 0 {
                eprintln!("{:#?} {:#?}", y, x);
            }
        }
    }

    // 不知道windows上会不会输出utf16
    #[cfg(target_family = "unix")]
    {
        let mut stdout = std::io::stdout();
        img.write_to(&mut stdout, image::ImageFormat::Png);
        stdout.flush();
    }

    // 不知道怎么搞定windows的情况，那就直接存个图吧
    #[cfg(target_family = "windows")]
    {
        img.save("image.png");
    }

    // for y in (0..height).rev() {
    //     for x in 0..width {
    //         let pixel = &buffer[y][x];
    //         println!(
    //             "{:?} {:?} {:?}",
    //             (pixel.r().sqrt() * 255.0).min(255.0) as usize,
    //             (pixel.g().sqrt() * 255.0).min(255.0) as usize,
    //             (pixel.b().sqrt() * 255.0).min(255.0) as usize, // 有时候会发现有的像素的rgb值超过了255
    //         );
    //     }
    // }
}

fn finalScene() -> Vec<Arc<dyn Bound<AxisAlignedBoundingBox>>> {
    let mut generator = thread_rng();

    // 地面凹凸不平的淡绿色方块地板
    let groundMaterial = Arc::new(Lambertian::new(Vec3::new(0.48, 0.83, 0.53)));

    let boxCountPerSide = 20;
    let mut cubes: Vec<Arc<dyn Bound<AxisAlignedBoundingBox>>> = vec![];

    for i in 0..boxCountPerSide {
        for j in 0..boxCountPerSide {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let y0 = 0.0;
            let z0 = -1000.0 + j as f64 * w;
            let x1 = x0 + w;
            let y1 = generator.gen_range(1.0, 101.0);
            let z1 = z0 + w;

            let cubeGeometry: Vec<Arc<dyn Bound<AxisAlignedBoundingBox>>> =
                Cube::new(x1 - x0, y1 - y0, z1 - z0)
                    .into_iter()
                    .map(|v| Arc::new(v) as Arc<dyn Bound<AxisAlignedBoundingBox>>)
                    .collect();
            let cubeGeometry = BoundingVolumeHierarchyNode::new(cubeGeometry).unwrap();
            let cube = Arc::new(
                Sprite::builder()
                    .geometry(Arc::new(cubeGeometry))
                    .material(groundMaterial.clone())
                    .transform(Mat4::translation(Vec3::new(
                        (x0 + x1) / 2.0,
                        (y0 + y1) / 2.0,
                        (z0 + z1) / 2.0,
                    )))
                    .build(),
            );

            cubes.push(cube);
        }
    }
    let cubes: Arc<dyn Bound<AxisAlignedBoundingBox>> =
        Arc::new(BoundingVolumeHierarchyNode::new(cubes).unwrap());

    let lightMaterial = Arc::new(DiffuseLight::new(Vec3::new(7.0, 7.0, 7.0)));
    let light = Arc::new(
        Sprite::builder()
            .geometry(Rectangle::new(300.0, 265.0).into())
            .material(lightMaterial)
            .transform(
                Mat4::translation(Vec3::new(273.0, 554.0, 279.5))
                    .multiplied(&Mat4::rotation(90.0_f64.to_radians(), Vec3::ex())),
            )
            .build(),
    );

    // 左上角的动态模糊，我还没写动态模糊
    let movingSphereMaterial = Arc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.1)));
    let movingSphere = Arc::new(
        Sprite::builder()
            .geometry(Sphere::new(50.0).into())
            .material(movingSphereMaterial)
            .transform(Mat4::translation(Vec3::new(400.0, 400.0, 200.0)))
            .build(),
    );

    // 正中间的玻璃球
    let glassSphere = Arc::new(
        Sprite::builder()
            .geometry(Sphere::new(50.0).into())
            .material(Dielectric::new(1.5).into())
            .transform(Mat4::translation(Vec3::new(260.0, 150.0, 45.0)))
            .build(),
    );
    // 书上的玻璃球不仅有折射，还有表面反射，我不知道他是怎么做出来的

    // 右下角的铁球
    let metalSphere = Arc::new(
        Sprite::builder()
            .geometry(Sphere::new(50.0).into())
            .material(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0).into())
            .transform(Mat4::translation(Vec3::new(0.0, 150.0, 145.0)))
            .build(),
    );

    // 不太理解这个东西，书上说是subsurface material，但是最后渲染出来是毛玻璃球的感觉，并不是书上那种陶瓷的感觉
    let blueSphereMaterial = Arc::new(Dielectric::new(1.5));
    let blueSphereSurface = Arc::new(
        Sprite::builder()
            .geometry(Sphere::new(70.0).into())
            .material(blueSphereMaterial)
            .transform(Mat4::translation(Vec3::new(360.0, 150.0, 145.0)))
            .build(),
    );
    let blueSphereMedium = Arc::new(
        Sprite::builder()
            .geometry(ConstantMedium::new(Sphere::new(70.0 - 1e-6).into(), 0.2).into())
            .material(Isotropic::new(Vec3::new(0.2, 0.4, 0.9)).into())
            .transform(Mat4::translation(Vec3::new(360.0, 150.0, 145.0)))
            .build(),
    );

    // 覆盖全景的雾，在靠近灯光的地方可以看到类似丁达尔效应的感觉（？）但是我没有
    let fog = Arc::new(
        Sprite::builder()
            .geometry(ConstantMedium::new(Sphere::new(5000.0).into(), 0.1).into())
            .material(Isotropic::new(Vec3::new(1.0, 1.0, 1.0)).into())
            .build(),
    );

    // 左中的地球
    let image = Arc::new(image::open("./earthmap.jpg").unwrap());
    let mapping = move |uv: &(f64, f64)| -> Vec3 {
        let image = image.clone();
        let buffer = image.as_rgb8().unwrap();
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
    let earthTexture: Arc<dyn Texture> = Arc::new(ImageTexture::new(mapping));
    let earthMaterial = Arc::new(Lambertian::new(earthTexture));
    let earth = Arc::new(
        Sprite::builder()
            .geometry(Sphere::new(100.0).into())
            .material(earthMaterial)
            .transform(Mat4::translation(Vec3::new(400.0, 200.0, 400.0)))
            .build(),
    );

    // 右上角的泡沫塑料
    let whiteMaterial = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let mut spheres: Vec<Arc<dyn Bound<AxisAlignedBoundingBox>>> = vec![];

    for _ in 0..1000 {
        let x = generator.gen_range(0.0, 165.0);
        let y = generator.gen_range(0.0, 165.0);
        let z = generator.gen_range(0.0, 165.0);

        let sphere = Arc::new(
            Sprite::builder()
                .geometry(Sphere::new(10.0).into())
                .material(whiteMaterial.clone())
                .transform(Mat4::translation(Vec3::new(
                    x - 100.0,
                    y + 270.0,
                    z + 395.0,
                )))
                .build(),
        );
        spheres.push(sphere);
    }
    let spheres: Arc<dyn Bound<AxisAlignedBoundingBox>> =
        Arc::new(BoundingVolumeHierarchyNode::new(spheres).unwrap());

    let res: Vec<Arc<dyn Bound<AxisAlignedBoundingBox>>> = vec![
        cubes,
        light as Arc<dyn Bound<AxisAlignedBoundingBox>>,
        movingSphere,
        glassSphere,
        metalSphere,
        blueSphereSurface,
        blueSphereMedium,
        earth,
        fog,
        spheres,
    ];

    return res;
}
