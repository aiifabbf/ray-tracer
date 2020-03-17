fn main() {
    let width = 200;
    let height = 100;
    println!("P3");
    println!("{:?} {:?}", width, height);
    println!("255");

    for y in (0..height).rev() {
        for x in 0..width {
            let r = x * 256 / width;
            let g = y * 256 / height;
            let b = (0.2f64 * 256f64) as usize;
            println!("{:?} {:?} {:?}", r, g, b);
        }
    }
}
