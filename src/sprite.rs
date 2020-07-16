use crate::mat4::Mat4;
use crate::mat4::Mat4Cached;
use crate::material::Material;
use crate::ray::Hit;
use crate::ray::HitRecord;
use crate::ray::Ray;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Sprite<T, U> {
    geometry: Option<Arc<T>>, // 这里好像就不得不用泛型了，纯粹的Hit无法保证这个Sprite对象能不能放到BVH里，但是又确实存在可能没有bounding box的sprite
    material: Option<Arc<U>>, // 还是把material改成泛型了……改成泛型之后出现了我没法理解的lifetime问题，还是暂时先不改了
    transform: Mat4Cached,
    // 书上只实现了translate和rotate，而且写的非常不漂亮……我就在想如何以不变应万变，如何做到任意4x4变换矩阵都可以。研究了一下书上translate和rotate的代码，我发现只要把输入光线做反变换、反射光线做正变换就可以了
    // 但是这里还是可能留下了两个问题：
    // 位移和旋转的行列式都是1，说明它们不改变原物体的体积，单位圆还是单位圆，但是如果det一旦不是1，比如一个把单位圆在垂直方向拉伸、变成椭球的矩阵，这时候表面法向量不是简单的做正变换，我记得是乘以逆变换的转置（但我找不到资料了）
    // 如何给变换后的物体生成bounding box
}

// 试试时髦的builder pattern？
pub struct SpriteBuilder<T, U> {
    sprite: Sprite<T, U>,
}

impl<T, U> SpriteBuilder<T, U> {
    pub fn build(self) -> Sprite<T, U> {
        return self.sprite;
    }

    pub fn geometry(mut self, geometry: Arc<T>) -> Self {
        self.sprite.geometry = Some(geometry);
        return self;
    }

    // pub fn material(mut self, material: Arc<dyn Material>) -> Self {
    //     self.sprite.material = Some(material);
    //     return self;
    // }

    pub fn material(mut self, material: Arc<U>) -> Self {
        self.sprite.material = Some(material);
        return self;
    }

    pub fn transform<M>(mut self, transform: M) -> Self
    where
        M: Into<Mat4Cached>,
    {
        self.sprite.transform = transform.into();
        return self;
    }
}

impl<T, U> Sprite<T, U> {
    pub fn builder() -> SpriteBuilder<T, U> {
        SpriteBuilder {
            sprite: Sprite {
                geometry: None,
                material: None,
                transform: Mat4::identity().into(),
            },
        }
    }

    pub fn new(geometry: Option<Arc<T>>, material: Option<Arc<U>>) -> Self {
        Self {
            geometry: geometry,
            material: material,
            transform: Mat4::identity().into(),
        }
    }

    pub fn geometry(&self) -> &Option<Arc<T>> {
        return &self.geometry;
    }

    pub fn material(&self) -> &Option<Arc<U>> {
        return &self.material;
    }

    pub fn transform(&self) -> &Mat4Cached {
        return &self.transform;
    }
}

impl<T, U> Hit for Sprite<T, U>
where
    T: Hit,
    U: Material + 'static,
{
    // <https://stackoverflow.com/questions/61712044/cast-arcrwlockt-to-arcrwlocktraitobject>
    // 放心了，'static并不是说在整个程序周期都有效，而是说可以放心的用，毕竟有Arc在，是不可能指向无效数据的
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        if let Some(geometry) = &self.geometry {
            // 既然geometry.rs里实现了(&Hit, &Mat4).hit()，那么这里其实只要这样写就可以了
            // return (geometry.as_ref(), self.transform()).hit(ray);

            if let Some(inversed) = &self.transform().inversed() {
                // 原光线反变换
                let origin = ray.origin().xyz1().transformed(inversed);
                let direction = ray.direction().xyz0().transformed(inversed);

                let ray = Ray::new(origin.into(), direction.into());

                if let Some(record) = geometry.hit(&ray) {
                    // 击中后再正变换
                    let intersection = record
                        .intersection()
                        .xyz1()
                        .transformed(self.transform().as_ref());
                    let normal = record
                        .normal()
                        .xyz0()
                        .transformed(self.transform().as_ref());

                    let res = HitRecord::new(
                        record.t(),
                        intersection.into(),
                        normal.into(),
                        // self.material
                        //     .as_ref()
                        //     .map(|v| v.clone() as Arc<dyn Material>),
                        self.material.as_ref().map(|v| v.as_ref() as &dyn Material),
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
