use crate::mat4::Mat4;
use crate::ray::Hit;
use crate::ray::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    radius: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Self {
        Self { radius: radius }
    }

    pub fn radius(&self) -> f64 {
        return self.radius;
    }

    // fn unitSphereUv(point: &Vec3) -> (f64, f64) {
    //     let phi = point.z().atan2(point.x());
    //     let theta = point.y().asin();
    //     let u = 1.0 - (phi + PI) / (2.0 * PI);
    //     let v = (theta + PI / 2.0) / PI;
    //     return (u, v);
    // }
    // 我觉得书上的uv有点怪，y+应该是朝上的

    // 和 <https://en.wikipedia.org/wiki/UV_mapping> 的还不太一样，v轴我这边和它是反过来的，这样的话地球贴图在球面上垂直方向才是正的
    fn unitSphereUv(point: &Vec3) -> (f64, f64) {
        let u = 0.5 + (point.x().atan2(point.z())) / (2.0 * PI);
        let v = 1.0 - point.y().acos() / PI; // acos的值域是[0, pi]
        return (u, v);
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let center = Vec3::new(0.0, 0.0, 0.0);
        let oc = *ray.origin() - center;
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
            let normal = ((intersection - center) / self.radius).normalized();
            let uv = Sphere::unitSphereUv(&((intersection - center) / self.radius()));
            let record = HitRecord::new(t, intersection, normal, None, uv);
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

// 终于到矩形了
#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
}
// 但我觉得这样的定义不太好，最好只要width和height，位移旋转都通过后面的transform来实现比较好
// 甚至还可以啥都不要，只给一个单位矩形，长宽都用变换实现
// 改好了

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width: width,
            height: height,
        }
    }

    pub fn width(&self) -> f64 {
        return self.width;
    }

    pub fn height(&self) -> f64 {
        return self.height;
    }
}

impl Hit for Rectangle {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let z = 0.0;
        let a = (-self.width / 2.0, -self.height / 2.0);
        let b = (self.width / 2.0, self.height / 2.0);

        let t = (z - ray.origin().z()) / ray.direction().z();
        if t.is_infinite() || t.is_nan() || t < 1e-6 {
            // 又被浮点数精度坑了……不长记性啊
            return None;
        }

        let x = ray.origin().x() + ray.direction().x() * t;
        let y = ray.origin().y() + ray.direction().y() * t;
        if x < a.0 || x > b.0 || y < a.1 || y > b.1 {
            return None;
        }

        let u = (x - a.0) / (b.0 - a.0);
        let v = (y - a.1) / (b.1 - a.1);

        return Some(HitRecord::new(
            t,
            ray.at(t),
            Vec3::new(0.0, 0.0, 1.0),
            None,
            (u, v),
        ));
    }
}

// 实现cube的时候突然发现有个麻烦，transform是只有sprite才有的性质，然而我这里想要用复合几何体，不需要material
impl<T> Hit for (T, Mat4)
where
    T: Hit,
{
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let geometry = &self.0;
        let transform = &self.1;

        if let Some(inversed) = &transform.inversed() {
            // 原光线反变换
            let origin = ray.origin().xyz1().transformed(inversed);
            let direction = ray.direction().xyz0().transformed(inversed);

            let ray = Ray::new(origin.into(), direction.into());

            if let Some(record) = geometry.hit(&ray) {
                // 击中后再正变换
                let intersection = record.intersection().xyz1().transformed(transform);
                let normal = record.normal().xyz0().transformed(transform);

                let res = HitRecord::new(
                    record.t(),
                    intersection.into(),
                    normal.into(),
                    None,
                    *record.uv(),
                );
                return Some(res);
            } else {
                return None;
            }
        } else {
            // det = 0，说明变换把物体直接拍扁了，这时候怎么处理呢
            return None;
        }
    }
}
// 直接把sprite那里的抄过来了。重构的时候要想想怎么用一份代码就行

#[derive(Clone, Debug)]
pub struct Cube;

impl Cube {
    pub fn new(width: f64, height: f64, depth: f64) -> Vec<(Rectangle, Mat4)> {
        return vec![
            (
                Rectangle::new(width, height),
                Mat4::translation(Vec3::new(0.0, 0.0, depth / 2.0)),
            ), // front
            (
                Rectangle::new(depth, height),
                Mat4::translation(Vec3::new(-width / 2.0, 0.0, 0.0))
                    .multiplied(&Mat4::rotation((-90.0 as f64).to_radians(), Vec3::ey())),
            ), // left
            (
                Rectangle::new(width, height),
                Mat4::translation(Vec3::new(0.0, 0.0, -depth / 2.0))
                    .multiplied(&Mat4::rotation((180.0 as f64).to_radians(), Vec3::ey())),
            ), // back
            (
                Rectangle::new(depth, height),
                Mat4::translation(Vec3::new(width / 2.0, 0.0, 0.0))
                    .multiplied(&Mat4::rotation((90.0 as f64).to_radians(), Vec3::ey())),
            ), // right
            (
                Rectangle::new(width, depth),
                Mat4::translation(Vec3::new(0.0, height / 2.0, 0.0))
                    .multiplied(&Mat4::rotation((-90.0 as f64).to_radians(), Vec3::ex())),
            ), // top
            (
                Rectangle::new(width, depth),
                Mat4::translation(Vec3::new(0.0, -height / 2.0, 0.0))
                    .multiplied(&Mat4::rotation((90.0 as f64).to_radians(), Vec3::ex())),
            ), // bottom
        ];
    }
}
