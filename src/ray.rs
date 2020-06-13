use crate::material::Material;
use crate::vec3::Vec3;

use std::sync::Arc;

#[derive(Copy, Debug, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin: origin,
            direction: direction,
        }
    }

    pub fn origin(&self) -> &Vec3 {
        // 这里如果返回Vec3会stackoverflow，也是活久见……
        return &self.origin;
    }

    pub fn direction(&self) -> &Vec3 {
        return &self.direction;
    }

    pub fn pointAtParameter(&self, t: f64) -> Vec3 {
        return *self.origin() + *self.direction() * t;
    }
}

pub struct HitRecord {
    t: f64,
    intersection: Vec3,
    normal: Vec3,
    material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn new(
        t: f64,
        intersection: Vec3,
        normal: Vec3,
        material: Option<Arc<dyn Material>>,
    ) -> Self {
        Self {
            t: t,
            intersection: intersection,
            normal: normal,
            material: material,
        }
    }

    pub fn t(&self) -> f64 {
        return self.t;
    }

    pub fn intersection(&self) -> &Vec3 {
        return &self.intersection;
    }

    pub fn normal(&self) -> &Vec3 {
        return &self.normal;
    }

    pub fn material(&self) -> &Option<Arc<dyn Material>> {
        return &self.material;
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}
