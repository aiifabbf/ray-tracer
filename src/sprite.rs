use crate::material::Material;
use crate::ray::Hit;
use crate::ray::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Sprite<T> {
    geometry: Option<Arc<T>>, // 这里好像就不得不用泛型了，纯粹的Hit无法保证这个Sprite对象能不能放到BVH里，但是又确实存在可能没有bounding box的sprite
    material: Option<Arc<dyn Material>>, // material暂时不改成泛型了吧……
}

impl<T> Sprite<T> {
    pub fn new(geometry: Option<Arc<T>>, material: Option<Arc<dyn Material>>) -> Self {
        Self {
            geometry: geometry,
            material: material,
        }
    }

    pub fn geometry(&self) -> &Option<Arc<T>> {
        return &self.geometry;
    }

    pub fn material(&self) -> &Option<Arc<dyn Material>> {
        return &self.material;
    }
}

impl<T> Hit for Sprite<T>
where
    T: Hit,
{
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        if let Some(geometry) = &self.geometry {
            if let Some(hitRecord) = geometry.hit(ray) {
                let res = HitRecord::new(
                    hitRecord.t(),
                    *hitRecord.intersection(),
                    *hitRecord.normal(),
                    self.material.clone(),
                );
                return Some(res);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
