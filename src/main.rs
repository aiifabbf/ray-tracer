mod vec3;

use vec3::Vec3;

fn main() {
    let width = 200;
    let height = 100;
    println!("P3");
    println!("{:?} {:?}", width, height);
    println!("255");

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = Vec3::new(x as f64 / width as f64, y as f64 / height as f64, 0.2);
            println!(
                "{:?} {:?} {:?}",
                (pixel.r() * 256.0) as usize,
                (pixel.g() * 256.0) as usize,
                (pixel.b() * 256.0) as usize,
            );
        }
    }
}
