use crate::mat4::Mat4;
use crate::vec3::Vec3;

use std::ops::*;

// 这太蠢了，每个都要写一遍

#[derive(Copy, Debug, Clone)]
pub struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn x(&self) -> f64 {
        return self.x;
    }

    pub fn y(&self) -> f64 {
        return self.y;
    }

    pub fn z(&self) -> f64 {
        return self.z;
    }

    pub fn w(&self) -> f64 {
        return self.w;
    }

    pub fn xyz(&self) -> Vec3 {
        return Vec3::new(self.x, self.y, self.z);
    }

    pub fn r(&self) -> f64 {
        return self.x;
    }

    pub fn g(&self) -> f64 {
        return self.y;
    }

    pub fn b(&self) -> f64 {
        return self.z;
    }

    pub fn a(&self) -> f64 {
        return self.w;
    }

    pub fn dot(&self, other: &Self) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
    }

    pub fn length(&self) -> f64 {
        return (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    pub fn normalized(&self) -> Self {
        return *self / self.length();
    }

    pub fn transformed(&self, transform: &Mat4) -> Self {
        let m = transform.as_slice();
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w = self.w;

        return Self {
            x: m[0] * x + m[4] * y + m[8] * z + m[12] * w,
            y: m[1] * x + m[5] * y + m[9] * z + m[13] * w,
            z: m[2] * x + m[6] * y + m[10] * z + m[14] * w,
            w: m[3] * x + m[7] * y + m[11] * z + m[15] * w,
        };
    }

    // Vec4还有反射和折射吗？
}

// Vec4变成Vec3
impl Into<Vec3> for Vec4 {
    fn into(self) -> Vec3 {
        Vec3::new(
            self.x, self.y, self.z, // 写self.z和写self.z()有区别吗，会变慢吗
        )
    }
}

// Vec3到Vec4其实是未定义的，谁知道w是0还是1呢？
// impl Into<Vec4> for Vec3 {
//     fn into(self) -> Vec4 {
//         Vec4 {
//             x: self.x(),
//             y: self.y(),
//             z: self.z(),
//             w: 0.0,
//         }
//     }
// }
// 所以我加了Vec3.xyz1和Vec3.xyz0
// w是0表示向量，w是1表示点
// 为什么是这样呢？可以这样理解：点+点没有意义，点+向量还是点，向量加向量还是向量，所以点的w是1、向量的w是0

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Add<f64> for Vec4 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }
}

impl AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Sub<f64> for Vec4 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
        }
    }
}

impl SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl Mul<f64> for Vec4 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Mul<Vec4> for f64 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
            w: other.w * self,
        }
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

impl Div<f64> for Vec4 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.z / other,
        }
    }
}

impl DivAssign<f64> for Vec4 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self.w /= other;
    }
}

impl Index<usize> for Vec4 {
    type Output = f64;

    fn index(&self, dimension: usize) -> &Self::Output {
        match dimension {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            v => panic!("dimension out of range: {}", v),
        }
    }
}
