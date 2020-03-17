use crate::vec3::Vec3;

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

    pub fn origin(&self) -> &Vec3 { // 这里如果返回Vec3会stackoverflow，也是活久见……
        return &self.origin;
    }

    pub fn direction(&self) -> &Vec3 {
        return &self.direction;
    }

    pub fn pointAtParameter(&self, t: f64) -> Vec3 {
        return *self.origin() + *self.direction() * t;
    }
}