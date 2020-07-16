use crate::vec3::Vec3;

use std::ops::*;

// 突然发现没办法抄gl-matrix的代码，因为gl-matrix的mat4是column-major的……

// 诶，那我也用column-major不就好了吗？

// 实在是不太喜欢column-major

// 好恶心啊，我想用nalgebra了

// 让Mat4实现Copy是不是开销有点大
#[derive(Copy, Debug, Clone)]
pub struct Mat4 {
    a: [f64; 16],
}
// 问了@luojia65大佬，曰编译器会优化成in-place，所以心安理得地Copy了

impl Mat4 {
    pub fn identity() -> Self {
        let mut a = [0.0; 16];
        a[0] = 1.0;
        a[5] = 1.0;
        a[10] = 1.0;
        a[15] = 1.0;
        return Self { a: a };
    }

    pub fn zero() -> Self {
        Self { a: [0.0; 16] }
    }

    // 构造平移变换矩阵
    // <http://glmatrix.net/docs/mat4.js.html#line874>
    pub fn translation(offset: Vec3) -> Self {
        let mut a = [0.0; 16];
        a[0] = 1.0;
        a[5] = 1.0;
        a[10] = 1.0;
        a[15] = 1.0;

        a[12] = offset[0];
        a[13] = offset[1];
        a[14] = offset[2];
        return Self { a: a };
    }

    // 构造旋转变换矩阵
    // <http://glmatrix.net/docs/mat4.js.html#line937>
    // <https://en.wikipedia.org/wiki/Rotation_matrix#Axis_and_angle>
    pub fn rotation(radians: f64, axis: Vec3) -> Self {
        let x = axis.x();
        let y = axis.y();
        let z = axis.z();

        let s = radians.sin();
        let c = radians.cos();
        let t = 1.0 - c;

        let a = [
            x * x * t + c,
            y * x * t + z * s,
            z * x * t - y * s,
            0.0,
            x * y * t - z * s,
            y * y * t + c,
            z * y * t + x * s,
            0.0,
            x * z * t + y * s,
            y * z * t - x * s,
            z * z * t + c,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ];
        return Self { a: a };
    }

    // 传统意义上的矩阵乘法（不是piecewise），注意是self * other，不是other * self
    // <http://glmatrix.net/docs/mat4.js.html#line502>
    // 矩阵乘向量的函数我写在Vec4.transformed里面了
    pub fn multiplied(&self, other: &Self) -> Self {
        let a00 = self.a[0];
        let a01 = self.a[1];
        let a02 = self.a[2];
        let a03 = self.a[3];

        let a10 = self.a[4];
        let a11 = self.a[5];
        let a12 = self.a[6];
        let a13 = self.a[7];

        let a20 = self.a[8];
        let a21 = self.a[9];
        let a22 = self.a[10];
        let a23 = self.a[11];

        let a30 = self.a[12];
        let a31 = self.a[13];
        let a32 = self.a[14];
        let a33 = self.a[15];

        let mut a = [0.0; 16];

        // 下面这段完全可以写成for啊……
        let mut b0 = other.a[0];
        let mut b1 = other.a[1];
        let mut b2 = other.a[2];
        let mut b3 = other.a[3];
        a[0] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        a[1] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        a[2] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        a[3] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;
        b0 = other.a[4];
        b1 = other.a[5];
        b2 = other.a[6];
        b3 = other.a[7];
        a[4] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        a[5] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        a[6] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        a[7] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;
        b0 = other.a[8];
        b1 = other.a[9];
        b2 = other.a[10];
        b3 = other.a[11];
        a[8] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        a[9] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        a[10] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        a[11] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;
        b0 = other.a[12];
        b1 = other.a[13];
        b2 = other.a[14];
        b3 = other.a[15];
        a[12] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        a[13] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        a[14] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        a[15] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

        return Self { a: a };
    }

    // <http://glmatrix.net/docs/mat4.js.html#line459>
    pub fn determinant(&self) -> f64 {
        let a00 = self.a[0];
        let a01 = self.a[1];
        let a02 = self.a[2];
        let a03 = self.a[3];

        let a10 = self.a[4];
        let a11 = self.a[5];
        let a12 = self.a[6];
        let a13 = self.a[7];

        let a20 = self.a[8];
        let a21 = self.a[9];
        let a22 = self.a[10];
        let a23 = self.a[11];

        let a30 = self.a[12];
        let a31 = self.a[13];
        let a32 = self.a[14];
        let a33 = self.a[15];

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        return b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
    }

    // 有可能是不可逆矩阵
    pub fn inversed(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }

        let a00 = self.a[0];
        let a01 = self.a[1];
        let a02 = self.a[2];
        let a03 = self.a[3];

        let a10 = self.a[4];
        let a11 = self.a[5];
        let a12 = self.a[6];
        let a13 = self.a[7];

        let a20 = self.a[8];
        let a21 = self.a[9];
        let a22 = self.a[10];
        let a23 = self.a[11];

        let a30 = self.a[12];
        let a31 = self.a[13];
        let a32 = self.a[14];
        let a33 = self.a[15];

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        let a = [
            (a11 * b11 - a12 * b10 + a13 * b09) / det,
            (a02 * b10 - a01 * b11 - a03 * b09) / det,
            (a31 * b05 - a32 * b04 + a33 * b03) / det,
            (a22 * b04 - a21 * b05 - a23 * b03) / det,
            (a12 * b08 - a10 * b11 - a13 * b07) / det,
            (a00 * b11 - a02 * b08 + a03 * b07) / det,
            (a32 * b02 - a30 * b05 - a33 * b01) / det,
            (a20 * b05 - a22 * b02 + a23 * b01) / det,
            (a10 * b10 - a11 * b08 + a13 * b06) / det,
            (a01 * b08 - a00 * b10 - a03 * b06) / det,
            (a30 * b04 - a31 * b02 + a33 * b00) / det,
            (a21 * b02 - a20 * b04 - a23 * b00) / det,
            (a11 * b07 - a10 * b09 - a12 * b06) / det,
            (a00 * b09 - a01 * b07 + a02 * b06) / det,
            (a31 * b01 - a30 * b03 - a32 * b00) / det,
            (a20 * b03 - a21 * b01 + a22 * b00) / det,
        ];

        return Some(Self { a: a });
    }

    // 不知道这个合不合规范
    pub fn as_slice(&self) -> &[f64] {
        return &self.a;
    }

    pub fn as_mut_slice(&mut self) -> &mut [f64] {
        return &mut self.a;
    }
}

impl Neg for Mat4 {
    type Output = Self;

    fn neg(mut self) -> Self {
        for v in self.a.iter_mut() {
            *v = -*v;
        }

        return self;
    }
}

impl Add<Mat4> for Mat4 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        for (v, w) in self.a.iter_mut().zip(other.a.iter()) {
            *v = *v + *w;
        }

        return self;
    }
}

impl Add<f64> for Mat4 {
    type Output = Self;

    fn add(mut self, other: f64) -> Self {
        for v in self.a.iter_mut() {
            *v = *v + other;
        }

        return self;
    }
}

impl Sub<Mat4> for Mat4 {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        for (v, w) in self.a.iter_mut().zip(other.a.iter()) {
            *v = *v - *w;
        }
        return self;
    }
}

impl Sub<f64> for Mat4 {
    type Output = Self;

    fn sub(mut self, other: f64) -> Self {
        for v in self.a.iter_mut() {
            *v = *v - other;
        }

        return self;
    }
}

impl SubAssign<Mat4> for Mat4 {
    fn sub_assign(&mut self, other: Self) {
        for (v, w) in self.a.iter_mut().zip(other.a.iter()) {
            *v = *v - *w;
        }
    }
}

impl Mul<f64> for Mat4 {
    type Output = Self;

    fn mul(mut self, other: f64) -> Self {
        for v in self.a.iter_mut() {
            *v = *v * other;
        }

        return self;
    }
}

impl Mul<Mat4> for f64 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut other = other;

        for v in other.a.iter_mut() {
            *v = *v * self;
        }

        return other;
    }
}

// 注意这是piecewise乘，矩阵乘
impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(mut self, other: Mat4) -> Mat4 {
        for (v, w) in self.a.iter_mut().zip(other.a.iter()) {
            *v = *v * *w;
        }

        return self;
    }
}

impl MulAssign<f64> for Mat4 {
    fn mul_assign(&mut self, other: f64) {
        for v in self.a.iter_mut() {
            *v = *v * other;
        }
    }
}

impl Div<f64> for Mat4 {
    type Output = Self;

    fn div(mut self, other: f64) -> Self {
        for v in self.a.iter_mut() {
            *v = *v / other;
        }

        return self;
    }
}

impl DivAssign<f64> for Mat4 {
    fn div_assign(&mut self, other: f64) {
        for v in self.a.iter_mut() {
            *v = *v / other;
        }
    }
}

// 这里index还是按照row-major来的，也就是说a[(1, 0)]还是取到的是第1行第0个元素
impl Index<(usize, usize)> for Mat4 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return &self.a[index.1 * 4 + index.0];
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mat4() {
        let mat = crate::mat4::Mat4::identity();
        println!("{:#?}", mat + mat);
        println!("{:#?}", mat * mat);
        println!("{:#?}", mat * 9.0);
        println!("{:#?}", mat * 9.0);
    }
}

// 加个cache吧，不然每次都要算逆矩阵真的太慢了……
// 一开始以为编译器会优化这个的，结果用flamegraph看了一下，发现算逆矩阵居然占用了大概三分之一的时间。
// 改好了之后再测试发现确实快了很多。
#[derive(Copy, Debug, Clone)]
pub struct Mat4Cached {
    origin: Mat4,
    inversed: Mat4,
    determinant: f64,
}

impl Mat4Cached {
    pub fn new(matrix: Mat4) -> Self {
        match matrix.inversed() {
            Some(inversed) => Self {
                origin: matrix,
                inversed: inversed,
                determinant: matrix.determinant(),
            },
            _ => Self {
                origin: matrix,
                inversed: Mat4::zero(),
                determinant: matrix.determinant(),
            },
        }
    }

    pub fn origin(&self) -> &Mat4 {
        &self.origin
    }

    pub fn inversed(&self) -> Option<&Mat4> {
        if self.determinant() == 0.0 {
            None
        } else {
            Some(&self.inversed)
        }
    }

    pub fn determinant(&self) -> f64 {
        self.determinant
    }
}

impl Into<Mat4> for Mat4Cached {
    fn into(self) -> Mat4 {
        *self.origin()
    }
}

impl Into<Mat4Cached> for Mat4 {
    fn into(self) -> Mat4Cached {
        Mat4Cached::new(self)
    }
}

impl AsRef<Mat4> for Mat4Cached {
    fn as_ref(&self) -> &Mat4 {
        self.origin()
    }
}
