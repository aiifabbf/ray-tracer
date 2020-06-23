use crate::ray::Hit;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn color(ray: &Ray, world: &dyn Hit, maxDepth: usize) -> Vec3 {
    if maxDepth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(record) = world.hit(ray) {
        if let Some(material) = record.material() {
            if let Some((scattered, attenuation)) = material.scatter(ray, &record) {
                return attenuation * color(&scattered, world, maxDepth - 1);
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        // 背景
        let unitDirection = ray.direction().normalized();
        let t = 0.5 * (unitDirection.y() + 1.0);
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}
