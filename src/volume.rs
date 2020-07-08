use crate::material::Isotropic;
use crate::material::Material;
use crate::material::Texture;
use crate::ray::Hit;
use crate::ray::HitRecord;
use crate::ray::Ray;
use crate::sprite::Sprite;
use crate::vec3::Vec3;

use rand::random;
use rand::thread_rng;
use rand::Rng;

use std::sync::Arc;

// 本来想把烟雾做成某种material，但是没有办法单独给带烟雾的Sprite实现Hit，所以烟雾只能变成某种geometry了
#[derive(Debug, Clone)]
pub struct ConstantMedium<T> {
    boundary: Arc<T>,
    density: f64,
}

impl<T> ConstantMedium<T> {
    pub fn new(boundary: Arc<T>, density: f64) -> Self {
        Self {
            boundary: boundary,
            density: density,
        }
    }

    pub fn boundary(&self) -> &Arc<T> {
        return &self.boundary;
    }

    pub fn density(&self) -> f64 {
        return self.density;
    }
}

impl<T> Hit for ConstantMedium<T>
where
    T: Hit,
{
    // 实现烟雾的大概思路是，不要在表面scatter，而是在物体的内部、在光束飞行的路线上随机选一点来scatter
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        if let Some(record1) = self.boundary.hit(&ray) {
            // 第一次hit
            if record1.normal().dot(ray.direction()) < 0.0 {
                // 第一次hit是进入物体
                let ray = Ray::new(
                    record1.intersection().clone() + *ray.direction() * 1e-6,
                    ray.direction().clone(),
                );
                if let Some(record2) = self.boundary.hit(&ray) {
                    // 第二次hit
                    let distanceInsideGeometry = record2.t(); // 光束在geometry内部飞行的距离
                    let mut generator = thread_rng();
                    let distance =
                        (-1.0 / self.density) * generator.gen_range(0.0 as f64, 1.0 as f64).ln(); // 为什么书这里要取log
                    if distance > distanceInsideGeometry {
                        return None;
                    }
                    let mut uv = *record1.uv();
                    uv.0 = uv.0 + record2.uv().0;
                    uv.1 = uv.1 + record2.uv().1;
                    return Some(HitRecord::new(
                        record1.t() + distance,
                        ray.at(record1.t() + distance),
                        (*record1.normal() + *record2.normal()) / 2.0,
                        None,
                        uv,
                    ));
                } else {
                    return None;
                }
            } else {
                // 第一次hit就是从物体中出去了，那么说明射线的起点本身就在物体内部
                let distanceInsideGeometry = record1.t(); // 光束在geometry内部飞行的距离
                let mut generator = thread_rng();
                let distance =
                    (-1.0 / self.density) * generator.gen_range(0.0 as f64, 1.0 as f64).ln(); // 为什么书这里要取log
                if distance > distanceInsideGeometry {
                    return None;
                }
                let mut uv = *record1.uv();
                uv.0 = uv.0;
                uv.1 = uv.1;
                return Some(HitRecord::new(
                    record1.t() + distance,
                    ray.at(record1.t() + distance),
                    record1.normal().clone(),
                    None,
                    uv,
                ));
            }
        } else {
            return None;
        }
    }
}
// 但是效果有点奇怪，方块的下半部分和示例不太一样。示例里面方块下半部分显得比较深，但是我却显得淡
