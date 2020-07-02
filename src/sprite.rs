use crate::mat4::Mat4;
use crate::material::Material;
use crate::ray::Hit;
use crate::ray::HitRecord;
use crate::ray::Ray;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Sprite<T> {
    geometry: Option<Arc<T>>, // 这里好像就不得不用泛型了，纯粹的Hit无法保证这个Sprite对象能不能放到BVH里，但是又确实存在可能没有bounding box的sprite
    material: Option<Arc<dyn Material>>, // 还是把material改成泛型了……改成泛型之后出现了我没法理解的lifetime问题，还是暂时先不改了
    transform: Mat4,
    // 书上只实现了translate和rotate，而且写的非常不漂亮……我就在想如何以不变应万变，如何做到任意4x4变换矩阵都可以。研究了一下书上translate和rotate的代码，我发现只要把输入光线做反变换、反射光线做正变换就可以了
    // 但是这里还是可能留下了两个问题：
    // 位移和旋转的行列式都是1，说明它们不改变原物体的体积，单位圆还是单位圆，但是如果det一旦不是1，比如一个把单位圆在垂直方向拉伸、变成椭球的矩阵，这时候表面法向量不是简单的做正变换，我记得是乘以逆变换的转置（但我找不到资料了）
    // 如何给变换后的物体生成bounding box
}

// 试试时髦的builder pattern？
pub struct SpriteBuilder<T> {
    sprite: Sprite<T>,
}

impl<T> SpriteBuilder<T> {
    pub fn build(self) -> Sprite<T> {
        return self.sprite;
    }

    pub fn geometry(mut self, geometry: impl Into<Arc<T>>) -> Self {
        self.sprite.geometry = Some(geometry.into());
        return self;
    }

    pub fn material(mut self, material: Arc<dyn Material>) -> Self {
        self.sprite.material = Some(material);
        return self;
    }

    pub fn transform(mut self, transform: Mat4) -> Self {
        self.sprite.transform = transform;
        return self;
    }
}

impl<T> Sprite<T> {
    pub fn builder() -> SpriteBuilder<T> {
        SpriteBuilder {
            sprite: Sprite {
                geometry: None,
                material: None,
                transform: Mat4::identity(),
            },
        }
    }

    pub fn new(geometry: Option<Arc<T>>, material: Option<Arc<dyn Material>>) -> Self {
        Self {
            geometry: geometry,
            material: material,
            transform: Mat4::identity(),
        }
    }

    pub fn geometry(&self) -> &Option<Arc<T>> {
        return &self.geometry;
    }

    pub fn material(&self) -> &Option<Arc<dyn Material>> {
        return &self.material;
    }

    pub fn transform(&self) -> &Mat4 {
        return &self.transform;
    }
}

impl<T> Hit for Sprite<T>
where
    T: Hit,
{
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        if let Some(geometry) = &self.geometry {
            if let Some(inversedTransform) = &self.transform().inversed() {
                // 原光线反变换
                let origin = ray.origin().xyz1().transformed(inversedTransform);
                let direction = ray.direction().xyz0().transformed(inversedTransform);

                let ray = Ray::new(origin.into(), direction.into());

                if let Some(record) = geometry.hit(&ray) {
                    // 击中后再正变换
                    let intersection = record.intersection().xyz1().transformed(self.transform());
                    let normal = record.normal().xyz0().transformed(self.transform());

                    let res = HitRecord::new(
                        record.t(),
                        // *record.intersection(),
                        intersection.into(),
                        // *record.normal(),
                        normal.into(),
                        self.material.clone(),
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
        } else {
            return None;
        }
    }
}
