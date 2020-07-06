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

#[derive(Clone)]
pub struct HitRecord<'a> {
    t: f64,
    intersection: Vec3,
    normal: Vec3,
    // material: Option<Arc<dyn Material>>,
    material: Option<&'a dyn Material>, // 能不能有一天改成ref呢
    uv: (f64, f64),
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f64,
        intersection: Vec3,
        normal: Vec3,
        // material: Option<Arc<dyn Material>>,
        material: Option<&'a dyn Material>,
        uv: (f64, f64), // 材质坐标
    ) -> Self {
        Self {
            t: t,
            intersection: intersection,
            normal: normal,
            material: material,
            uv: uv,
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

    // pub fn material(&self) -> &Option<Arc<dyn Material>> {
    pub fn material(&self) -> &Option<&dyn Material> {
        return &self.material;
    }

    pub fn uv(&self) -> &(f64, f64) {
        return &self.uv;
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}

impl Default for HitRecord<'_> {
    fn default() -> Self {
        Self {
            t: 1.0 / 0.0, // 这里必须是inf，因为bounding box只用来判断有没有hit，不判断hit在哪里
            intersection: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: None,
            uv: (0.0, 0.0),
        }
    }
}
