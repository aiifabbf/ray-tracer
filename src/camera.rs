use crate::ray::Ray;
use crate::util::randomInUnitDisk;
use crate::vec3::Vec3;

pub trait Camera: Send + Sync {
    fn ray(&self, u: f64, v: f64) -> Ray;
}

#[derive(Clone, Debug)]
pub struct PerspectiveCamera {
    eye: Vec3,          // 相机所在位置坐标
    center: Vec3,       // 目光终点坐标
    up: Vec3,           // 相机上方方向向量
    fov: f64,           // 垂直方向视角大小，弧度
    aspect: f64,        // 相机视窗宽度高度比值
    focusDistance: f64, // 对焦平面的距离。这就是znear吗？好像不是
    lensRadius: f64,    // 光圈半径
    lowerLeft: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}
// 觉得这个相机太重了……

impl PerspectiveCamera {
    pub fn new(
        eye: Vec3,
        center: Vec3,
        up: Vec3,
        fov: f64,
        aspect: f64,
        focusDistance: f64,
        lensRadius: f64,
    ) -> Self {
        let up = up.normalized();
        let height = (fov / 2.0).tan() * 2.0;
        let width = aspect * height;

        let w = (eye - center).normalized();
        let u = up.cross(&w);
        let v = w.cross(&u);

        // let lowerLeft = Vec3::new(-halfWidth, -halfHeight, -1.0); // -1.0这个是不是znear啊，那是不是应该能让用户调
        let horizontal = u * width * focusDistance;
        let vertical = v * height * focusDistance;
        let lowerLeft = eye - horizontal / 2.0 - vertical / 2.0 - w * focusDistance;

        Self {
            eye: eye,
            center: center,
            up: up,
            fov: fov,
            aspect: aspect,
            focusDistance: focusDistance,
            lensRadius: lensRadius,
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

    pub fn focusDistance(&self) -> f64 {
        return self.focusDistance;
    }

    pub fn lensRadius(&self) -> f64 {
        return self.lensRadius;
    }
}

impl Camera for PerspectiveCamera {
    fn ray(&self, u: f64, v: f64) -> Ray {
        if self.lensRadius == 0.0 {
            return Ray::new(
                self.eye,
                (self.lowerLeft + self.horizontal * u + self.vertical * v - self.eye).normalized(), // 这里direction要不要normalize呢……如果normalize，有一个好处是t就有非常明确的物理含义了，如果我们算出射线上某个点的t，就能确定这个点离射线的起点正好是t米
            );
        } else {
            let rd = self.lensRadius * randomInUnitDisk();
            let offset = rd.x() * u + rd.y() * v;
            return Ray::new(
                self.eye + offset,
                (self.lowerLeft + self.horizontal * u + self.vertical * v - self.eye - offset)
                    .normalized(),
            );
        }
    }
}
