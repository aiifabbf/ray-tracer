use crate::ray::Hit;
use crate::ray::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

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
            let mut t = t1; // 提示我value never read。可是我讨厌只声明不赋值
            if t1 > 0.0 {
                t = t1;
            } else if t2 > 0.0 {
                t = t2;
            } else {
                return None;
            }
            let intersection = ray.at(t);
            let normal = ((intersection - self.center) / self.radius).normalized();
            let front = if ray.direction().dot(&normal) > 0.0 {
                false
            } else {
                true
            };
            let record = HitRecord::new(t, intersection, normal, front, None);
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
                    if record.t() < res.as_ref().unwrap().t() {
                        res.replace(record);
                    }
                }
            }
        }

        return res;
    }
}

// 这里Send + Sync不知道怎么去掉，只能复读一遍了。很奇怪，dyn Hit + Send + Sync不能cast到dyn Hit。按理说dyn Hit + Send + Sync应该是dyn Hit的子集，那么cast到dyn Hit应该完全没问题
impl Hit for Vec<Box<dyn Hit + Send + Sync>> {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut res = None;

        for v in self.iter() {
            let record = v.hit(ray);
            if record.is_some() {
                let record = record.unwrap();
                if res.is_none() {
                    res.replace(record);
                } else {
                    if record.t() < res.as_ref().unwrap().t() {
                        res.replace(record);
                    }
                }
            }
        }

        return res;
    }
}
