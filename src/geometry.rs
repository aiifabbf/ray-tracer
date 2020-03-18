use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::ray::Hit;
use crate::ray::HitRecord;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {
            center: center,
            radius: radius,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction()) * 2.0;
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            let mut t;
            if t1 >= 0.0 {
                t = t1;
            } else if t2 >= 0.0 {
                t = t2;
            } else {
                return None;
            }
            let intersection = ray.pointAtParameter(t);
            let normal = (intersection - self.center) / self.radius;
            let record = HitRecord::new(t, intersection, normal);
            return Some(record);
        }
    }
}

impl Hit for Vec<Box<dyn Hit>> {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut res = None;

        for v in self.iter() {
            let record = v.hit(ray);
            if record.is_some() {
                let record = record.unwrap();
                if res.is_none() {
                    res.replace(record);
                } else {
                    if record.t() < res.unwrap().t() {
                        res.replace(record);
                    }
                }
            }
        }

        return res;
    }
}