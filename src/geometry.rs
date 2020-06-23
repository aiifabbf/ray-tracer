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

    pub fn center(&self) -> &Vec3 {
        return &self.center;
    }

    pub fn radius(&self) -> f64 {
        return self.radius;
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
            let (t1, t2) = if t1 < t2 { (t1, t2) } else { (t2, t1) };
            let mut t = t1; // 提示我value never read。可是我讨厌只声明不赋值
            if t1 > (10.0 as f64).powf(-6.0) {
                // 这里被坑惨了，千万不能直接判断大于0
                // 因为浮点数精度的问题，有时候射线的起点会偏移到球的内部
                t = t1;
            } else if t2 > (10.0 as f64).powf(-6.0) {
                t = t2;
            } else {
                return None;
            }
            let intersection = ray.at(t);
            let normal = ((intersection - self.center) / self.radius).normalized();
            let record = HitRecord::new(t, intersection, normal, None);
            return Some(record);
        }
    }
}

impl Hit for Vec<Box<dyn Hit>> {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut res = None;

        for v in self.iter() {
            if let Some(record) = v.hit(ray) {
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
// 去掉了impl Hit for Vec<Box<dyn Hit + Send + Sync>>，解决方法是直接声明Hit trait是Send + Sync trait的子集。

// 但是我总觉得哪里怪怪的，Arc并不能使本身不Send也不Sync的对象变得Send和Sync，但是RwLock能使本身不Sync的对象变得Sync。
// 我错了，RwLock<T>要求T本身是Send + Sync的，然后RwLock<T>整个变成Send + Sync，只有Mutex<T>只要求T本身只要Send就可以让Mutex<T>整个变得Send + Sync。
// 这里有一个解释 <https://stackoverflow.com/questions/50704279/when-or-why-should-i-use-a-mutex-over-an-rwlock>
