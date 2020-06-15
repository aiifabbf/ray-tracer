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
        // 后来发现并不是这个问题……是color()那里递归深度太大了
        return &self.origin;
    }

    pub fn direction(&self) -> &Vec3 {
        return &self.direction;
    }

    pub fn at(&self, t: f64) -> Vec3 {
        return *self.origin() + *self.direction() * t;
    }
}

pub struct HitRecord {
    t: f64,
    intersection: Vec3,
    normal: Vec3,
    front: bool, // true说明这道光线射中的是物体的外表面
    material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn new(
        t: f64,
        intersection: Vec3,
        normal: Vec3,
        front: bool,
        material: Option<Arc<dyn Material>>,
    ) -> Self {
        Self {
            t: t,
            intersection: intersection,
            normal: normal,
            front: front,
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

    pub fn front(&self) -> bool {
        return self.front;
    }

    pub fn material(&self) -> &Option<Arc<dyn Material>> {
        return &self.material;
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}
