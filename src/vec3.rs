use std::ops::*;

#[derive(Copy, Debug, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
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

    pub fn r(&self) -> f64 {
        return self.x;
    }

    pub fn g(&self) -> f64 {
        return self.y;
    }

    pub fn b(&self) -> f64 {
        return self.z;
    }

    pub fn dot(&self, other: &Self) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f64 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }

    pub fn normalize(&mut self) {
        *self /= self.length(); // 这里我不懂为啥一定要*self
    }

    pub fn normalized(&self) -> Self {
        return *self / self.length();
    }

    pub fn reflected(&self, normal: &Self) -> Self {
        return *self - *normal * self.dot(&normal) * 2.0;
    }

    // pub fn refracted(&self, normal: &Self, refractiveInOverOut: f64) -> Self {
    //     let normalizedSelf = self.normalized();
    //     let cosTheta = -normalizedSelf.dot(normal);
    //     let parallel = refractiveInOverOut * (normalizedSelf + *normal * cosTheta);
    //     let perpendicular = -(1.0 - parallel.length().sqrt()).sqrt() * *normal;
    //     return parallel + perpendicular;
    // }

    // refract不一定总是有解的，比如从水里往空气里射一束光线，如果入射光足够贴近界面，是不会有折射的，会发生全反射的
    pub fn refracted(&self, normal: &Self, refractiveInOverOut: f64) -> Option<Self> {
        let uv = self.normalized();
        let dt = uv.dot(normal);
        let discriminant = 1.0 - refractiveInOverOut * refractiveInOverOut * (1.0 - dt * dt);
        if discriminant > 0.0 {
            return Some(refractiveInOverOut * (*self - *normal * dt) - *normal * discriminant.sqrt());
        } else {
            return None;
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// 要不要写impl Add<&Vec3> for &Vec3呢……可是这样每个都要写一遍，好麻烦

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}
