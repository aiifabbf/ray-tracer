use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Camera: Send + Sync {
    fn ray(&self, u: f64, v: f64) -> Ray;
}

#[derive(Clone, Debug)]
pub struct PerspectiveCamera {
    eye: Vec3,    // 相机所在位置坐标
    center: Vec3, // 目光终点坐标
    up: Vec3,     // 相机上方方向向量
    fov: f64,     // 垂直方向视角大小，弧度
    aspect: f64,  // 相机视窗宽度高度比值
    lowerLeft: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl PerspectiveCamera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3, fov: f64, aspect: f64) -> Self {
        let up = up.normalized();
        let halfHeight = (fov / 2.0).tan();
        let halfWidth = aspect * halfHeight;

        let w = (eye - center).normalized();
        let u = up.cross(&w);
        let v = w.cross(&u);

        // let lowerLeft = Vec3::new(-halfWidth, -halfHeight, -1.0); // -1.0这个是不是znear啊，那是不是应该能让用户调
        let horizontal = u * halfWidth * 2.0;
        let vertical = v * halfHeight * 2.0;
        let lowerLeft = eye - u * halfWidth - v * halfHeight - w;

        Self {
            eye: eye,
            center: center,
            up: up,
            fov: fov,
            aspect: aspect,
            lowerLeft: lowerLeft,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn eye(&self) -> &Vec3 {
        return &self.eye;
    }

    pub fn center(&self) -> &Vec3 {
        return &self.center;
    }

    pub fn up(&self) -> &Vec3 {
        return &self.up;
    }

    pub fn fov(&self) -> f64 {
        return self.fov;
    }

    pub fn aspect(&self) -> f64 {
        return self.aspect;
    }
}

impl Camera for PerspectiveCamera {
    fn ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.eye,
            (self.lowerLeft + self.horizontal * u + self.vertical * v - self.eye).normalized(), // 这里direction要不要normalize呢……
        );
    }
}
