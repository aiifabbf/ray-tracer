use crate::material::Material;
use crate::ray::Hit;
use crate::ray::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sprite {
    geometry: Option<Arc<dyn Hit>>,
    material: Option<Arc<dyn Material>>,
}

impl Sprite {
    pub fn new(geometry: Option<Arc<dyn Hit>>, material: Option<Arc<dyn Material>>) -> Self {
        Self {
            geometry: geometry,
            material: material,
        }
    }
}

impl Hit for Sprite {
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
