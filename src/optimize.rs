use crate::geometry::Rectangle;
use crate::geometry::Sphere;
use crate::geometry::TransformedGeometry;
use crate::material::Material;
use crate::ray::Hit;
use crate::ray::HitRecord;
use crate::ray::Ray;
use crate::sprite::Sprite;
use crate::vec3::Vec3;
use crate::volume::ConstantMedium;

use rand::thread_rng;
use rand::Rng;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem::swap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AxisAlignedBoundingBox {
    min: Vec3, // 方块上坐标三个维度都是最小的那个点，能不能是-inf, -inf, -inf呢……
    max: Vec3, // 方块上坐标三个维度都是最大的那个点，能不能是inf, inf, inf呢……
}

impl AxisAlignedBoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        // debug_assert!(max.x() > min.x()); // 能不能等于呢
        // debug_assert!(max.y() > min.y());
        // debug_assert!(max.z() > min.z());
        // debug_assert!(!min.isNan());
        // debug_assert!(!max.isNan());
        Self { min: min, max: max }
    }

    pub fn min(&self) -> &Vec3 {
        return &self.min;
    }

    pub fn max(&self) -> &Vec3 {
        return &self.max;
    }

    // 和另一个AABB合并成一个最小的AABB
    pub fn merged(&self, other: &Self) -> Self {
        let min = Vec3::new(
            self.min().x().min(other.min().x()),
            self.min().y().min(other.min().y()),
            self.min().z().min(other.min().z()),
        );
        let max = Vec3::new(
            self.max().x().max(other.max().x()),
            self.max().y().max(other.max().y()),
            self.max().z().max(other.max().z()),
        );
        return AxisAlignedBoundingBox::new(min, max);
    }
}

impl Hit for AxisAlignedBoundingBox {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        // 这种实现我觉得并不是很直观……但是好像可以避免nan的问题
        let mut tmin = 0.0;
        let mut tmax = 1.0 / 0.0; // inf

        for i in 0..3 {
            let inv = 1.0 / ray.direction()[i]; // 如果某一维是0，那么inv会变成inf
            let mut t0 = (self.min[i] - ray.origin()[i]) * inv;
            let mut t1 = (self.max[i] - ray.origin()[i]) * inv;
            if inv < 0.0 {
                // 危险，这里要不要改成1e-6呢
                swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return None;
            }
        }

        return Some(HitRecord::default()); // 这里返回了一个空的HitRecord，可能以后能加点优化？
    }
}

// Bound trait表示某个对象有可能能用某种bounding box包裹起来
// 有个T是因为可能不止能被AABB包裹，以后可能还有别的bounding的方法
// 所以对象如果能被AABB包裹，那么对象就实现Bound<AABB>
pub trait Bound<T>: Send + Sync + Hit + Debug
where
    T: Hit,
{
    fn bound(&self) -> Option<T>;
    // 这个Option是临时加的，因为下面写Vec的impl的时候突然发现，万一Vec是空的，那么bounding box岂不是不存在，有两个方案，第一个是直接让AABB变成以原点开始、边长全是0的；另一个方案就是用Option
    // 为什么空的AABB就非要从原点开始呢？所以就选了Option这个方案
}

// 自己被自己包裹
impl Bound<AxisAlignedBoundingBox> for AxisAlignedBoundingBox {
    fn bound(&self) -> Option<AxisAlignedBoundingBox> {
        return Some(self.clone());
    }
}

impl Bound<AxisAlignedBoundingBox> for Sphere {
    fn bound(&self) -> Option<AxisAlignedBoundingBox> {
        let center = Vec3::new(0.0, 0.0, 0.0);

        return Some(AxisAlignedBoundingBox::new(
            center - Vec3::new(self.radius(), self.radius(), self.radius()),
            center + Vec3::new(self.radius(), self.radius(), self.radius()),
        ));
    }
}

impl Bound<AxisAlignedBoundingBox> for Rectangle {
    fn bound(&self) -> Option<AxisAlignedBoundingBox> {
        let z = 0.0;
        let a = (-self.width() / 2.0, -self.height() / 2.0);
        let b = (self.width() / 2.0, self.height() / 2.0);

        return Some(AxisAlignedBoundingBox::new(
            Vec3::new(a.0, a.1, z - 1e-6),
            Vec3::new(b.0, b.1, z + 1e+6),
        ));
    }
}

impl<T, U> Bound<AxisAlignedBoundingBox> for Sprite<T, U>
where
    T: Bound<AxisAlignedBoundingBox>,
    U: Material + 'static, // 可是这里根本和material没关系啊
{
    fn bound(&self) -> Option<AxisAlignedBoundingBox> {
        if let Some(geometry) = self.geometry() {
            // TODO: 现在有了transform，这边要改
            let transform = self.transform();

            if let Some(bound) = geometry.bound() {
                let x0 = bound.min()[0];
                let y0 = bound.min()[1];
                let z0 = bound.min()[2];

                let x1 = bound.max()[0];
                let y1 = bound.max()[1];
                let z1 = bound.max()[2];

                let mut min = vec![1.0 / 0.0; 3];
                let mut max = vec![-1.0 / 0.0; 3];

                for point in [
                    Vec3::new(x0, y0, z0),
                    Vec3::new(x1, y0, z0),
                    Vec3::new(x0, y1, z0),
                    Vec3::new(x0, y0, z1),
                    Vec3::new(x1, y1, z0),
                    Vec3::new(x1, y0, z1),
                    Vec3::new(x0, y1, z1),
                    Vec3::new(x1, y1, z1),
                ]
                .iter()
                .map(|v| v.xyz1().transformed(transform).xyz())
                {
                    for i in 0..3 {
                        if point[i] < min[i] {
                            min[i] = point[i];
                        }

                        if point[i] > max[i] {
                            max[i] = point[i];
                        }
                    }
                }

                let bound = AxisAlignedBoundingBox::new(
                    Vec3::new(min[0], min[1], min[2]),
                    Vec3::new(max[0], max[1], max[2]),
                );
                return Some(bound);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}

// 这里怎么又重复了一遍
impl<T> Bound<AxisAlignedBoundingBox> for TransformedGeometry<T>
where
    T: Bound<AxisAlignedBoundingBox>,
{
    fn bound(&self) -> Option<AxisAlignedBoundingBox> {
        let geometry = self.geometry();
        let transform = self.transform();

        if let Some(bound) = geometry.bound() {
            let x0 = bound.min()[0];
            let y0 = bound.min()[1];
            let z0 = bound.min()[2];

            let x1 = bound.max()[0];
            let y1 = bound.max()[1];
            let z1 = bound.max()[2];

            let mut min = vec![1.0 / 0.0; 3];
            let mut max = vec![-1.0 / 0.0; 3];

            for point in [
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y0, z0),
                Vec3::new(x0, y1, z0),
                Vec3::new(x0, y0, z1),
                Vec3::new(x1, y1, z0),
                Vec3::new(x1, y0, z1),
                Vec3::new(x0, y1, z1),
                Vec3::new(x1, y1, z1),
            ]
            .iter()
            .map(|v| v.xyz1().transformed(transform).xyz())
            {
                for i in 0..3 {
                    if point[i] < min[i] {
                        min[i] = point[i];
                    }

                    if point[i] > max[i] {
                        max[i] = point[i];
                    }
                }
            }

            let bound = AxisAlignedBoundingBox::new(
                Vec3::new(min[0], min[1], min[2]),
                Vec3::new(max[0], max[1], max[2]),
            );
            return Some(bound);
        } else {
            return None;
        }
    }
}

// 可以针对球特殊优化
// impl Bound<AxisAlignedBoundingBox> for TransformedGeometry<Sphere> {
//     fn bound(&self) -> Option<AxisAlignedBoundingBox> {}
// }

// 一开始想这样写的……但是遇到了超级多的麻烦
// impl<T> Bound<AxisAlignedBoundingBox> for (&T, &Mat4)
// where
//     T: Bound<AxisAlignedBoundingBox>,
// {
//     fn bound(&self) -> Option<AxisAlignedBoundingBox> {}
// }

impl Bound<AxisAlignedBoundingBox> for Vec<Box<dyn Bound<AxisAlignedBoundingBox>>> {
    fn bound(&self) -> Option<AxisAlignedBoundingBox> {
        let mut res = Option::<AxisAlignedBoundingBox>::None;

        for v in self.iter() {
            if let Some(other) = &v.bound() {
                if let Some(bound) = &res {
                    res = Some(bound.merged(other));
                } else {
                    res = Some(other.clone());
                }
            }
        }

        return res;
    }
}
// 不写impl Hit for Vec<Box<dyn Bound<AxisAlignedBoundingBox>>>的话会提示Vec<Box<dyn Bound<AxisAlignedBoundingBox>>>不满足Hit……很迷

// 这里怎么又要写一遍……明明Vec<Box<dyn Hit>>一定满足Hit、Bound<AABB>又是Hit的，说明Vec<Box<dyn Bound<AABB>>>肯定是Vec<Box<dyn Hit>>的子集，为啥还要写一遍呢……
impl Hit for Vec<Box<dyn Bound<AxisAlignedBoundingBox>>> {
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

// 重头戏，AABB组成的BVH
// 这边想了很久，rust有个限制是不能dyn A + B + C
#[derive(Clone, Debug)]
pub struct BoundingVolumeHierarchyNode<T> {
    // box是个保留字，所以用volume了
    volume: T,
    left: Option<Arc<dyn Bound<T>>>, // 有没有可能自己是AABB，但是left和right确实其他类型的bounding box呢？如果是这样的话，那泛型是不是要写成<T, U, V>了……
    right: Option<Arc<dyn Bound<T>>>, // 先别想这么多吧……
}

impl<T> BoundingVolumeHierarchyNode<T>
where
    T: Bound<T>, // Bound<T>已经包含了Hit
{
    pub fn volume(&self) -> &T {
        return &self.volume;
    }

    pub fn left(&self) -> &Option<Arc<dyn Bound<T>>> {
        return &self.left;
    }

    pub fn right(&self) -> &Option<Arc<dyn Bound<T>>> {
        return &self.right;
    }
}

impl BoundingVolumeHierarchyNode<AxisAlignedBoundingBox> {
    // 这里没法传入&[Arc<dyn Bound<_>>]，因为下面要sort，只能move进来了
    // 那能不能传入Vec<&dyn Bound<_>>呢，那可能要标记lifetime了
    pub fn new(objects: Vec<Arc<dyn Bound<AxisAlignedBoundingBox>>>) -> Option<Self> {
        if objects.is_empty() {
            // 空的怎么办……
            return None;
        }

        let mut objects = objects;

        let mut generator = thread_rng();
        let axis = generator.gen_range(0, 3);
        let compare = match axis {
            0 => Self::compareX,
            1 => Self::compareY,
            _ => Self::compareZ,
        };

        let mut left = None;
        let mut right = None;

        if objects.len() == 1 {
            left = Some(objects[0].clone());
        } else if objects.len() == 2 {
            if compare(objects[0].as_ref(), objects[1].as_ref()) {
                left = Some(objects[0].clone());
                right = Some(objects[1].clone());
            } else {
                left = Some(objects[1].clone());
                right = Some(objects[0].clone());
            }
        } else {
            objects.sort_by(|v, w| {
                if compare(v.as_ref(), w.as_ref()) {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }); // 为什么&Arc<dyn ...>不会自动cast到&dyn ...？
            let middle = objects.len() / 2;
            right = Self::new(objects.split_off(middle))
                .map(|v| Arc::new(v) as Arc<dyn Bound<AxisAlignedBoundingBox>>);
            // split_off()会把vec分成两个vec，返回右半边，原来的被截断到左半边
            left =
                Self::new(objects).map(|v| Arc::new(v) as Arc<dyn Bound<AxisAlignedBoundingBox>>);
            // 这好难看啊
        }

        let mut leftVolume = None;

        if let Some(node) = &left {
            leftVolume = node.bound();
        }

        let mut rightVolume = None;

        if let Some(node) = &right {
            rightVolume = node.bound();
        }

        let volume = match (leftVolume, rightVolume) {
            (Some(v), Some(w)) => Some(v.merged(&w)),
            (Some(v), None) => Some(v),
            (None, Some(w)) => Some(w),
            _ => None,
        };

        if let Some(v) = volume {
            return Some(Self {
                volume: v,
                left: left,
                right: right,
            });
        } else {
            return None;
        }
    }

    fn compareX(
        v: &dyn Bound<AxisAlignedBoundingBox>,
        w: &dyn Bound<AxisAlignedBoundingBox>,
    ) -> bool {
        v.bound().unwrap().min()[0] < w.bound().unwrap().min()[0]
    }

    fn compareY(
        v: &dyn Bound<AxisAlignedBoundingBox>,
        w: &dyn Bound<AxisAlignedBoundingBox>,
    ) -> bool {
        v.bound().unwrap().min()[1] < w.bound().unwrap().min()[1]
    }

    fn compareZ(
        v: &dyn Bound<AxisAlignedBoundingBox>,
        w: &dyn Bound<AxisAlignedBoundingBox>,
    ) -> bool {
        v.bound().unwrap().min()[2] < w.bound().unwrap().min()[2]
    }
}

impl<T> Hit for BoundingVolumeHierarchyNode<T>
where
    T: Bound<T>,
{
    // 递归的写法。什么时候试下BFS
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        if let Some(record) = self.volume.hit(ray) {
            let mut record = record;

            if let Some(left) = &self.left {
                if let Some(leftRecord) = left.hit(ray) {
                    if leftRecord.t() < record.t() {
                        record = leftRecord;
                    }
                }
            }

            if let Some(right) = &self.right {
                if let Some(rightRecord) = right.hit(ray) {
                    if rightRecord.t() < record.t() {
                        record = rightRecord;
                    }
                }
            }

            if record.t().is_infinite() {
                return None;
            } else {
                return Some(record);
            }
        } else {
            return None;
        }
    }
    // 好像并没有办法用BFS，因为left和right不一定是node，可能是普通的geometry了
}

impl Bound<AxisAlignedBoundingBox> for BoundingVolumeHierarchyNode<AxisAlignedBoundingBox> {
    fn bound(&self) -> Option<AxisAlignedBoundingBox> {
        return Some(self.volume.clone());
    }
}

impl<T, B> Bound<B> for ConstantMedium<T>
where
    T: Bound<B>,
    B: Hit,
{
    fn bound(&self) -> Option<B> {
        return self.geometry().bound();
    }
}
