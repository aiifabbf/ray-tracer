use crate::vec3::Vec3;
use rand::random;
use rand::thread_rng;
use rand::Rng;

pub fn randomInUnitSphere() -> Vec3 {
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

pub fn randomInUnitDisk() -> Vec3 {
    let mut generator = thread_rng();

    loop {
        let p = Vec3::new(
            generator.gen_range(-1.0, 1.0),
            generator.gen_range(-1.0, 1.0),
            0.0,
        );
        if p.length() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}
