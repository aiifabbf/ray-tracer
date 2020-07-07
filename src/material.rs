use crate::ray::HitRecord;
use crate::ray::Ray;
use crate::util::randomInUnitSphere;
use crate::vec3::Vec3;

use std::f64::consts::PI;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, rayIn: &Ray, hitRecord: &HitRecord) -> Option<(Ray, Vec3)>;
    // 我觉得这里有点问题，真实世界里一束入射光会散射出多束反射光，但是这里只会返回一束光，以后怎么扩展成多束光呢

    fn emitted(&self, uv: &(f64, f64), point: &Vec3) -> Vec3 {
        // 这里纠结了一下要不要搞成Option<Vec3>
        return Vec3::new(0.0, 0.0, 0.0); // 默认不发光
    }
}

#[derive(Clone, Debug)]
pub struct Lambertian {
    // 改成Arc<dyn Texture>之后破坏了原来的API，怎么办
    albedo: Arc<dyn Texture>, // 把材质直接写在material里有点奇怪……
}

// 所以这里把new()变成了泛型方法，就可以既接受Vec3表示的颜色，又可以接受其他dyn Texture
// Rust不支持function overloading，所以这样也算是变相实现了overloading吧
// 还有一种方案是这边不要泛型，让用户在外面手动vec3.into()
impl Lambertian {
    pub fn new<T>(albedo: T) -> Self
    where
        T: Into<Arc<dyn Texture>>,
    {
        Self {
            albedo: albedo.into(),
        }
    }

    pub fn albedo(&self) -> &Arc<dyn Texture> {
        return &self.albedo;
    }
}

// 并且在这里把Vec3实现为Into<Arc<dyn Texture>>，这样普通的Vec3就可以塞到Lambertian::new()里面了
impl Into<Arc<dyn Texture>> for Vec3 {
    fn into(self) -> Arc<dyn Texture> {
        Arc::new(SolidColor { color: self })
    }
}

impl Into<SolidColor> for Vec3 {
    fn into(self) -> SolidColor {
        SolidColor { color: self }
    }
}

impl Material for Lambertian {
    fn scatter(&self, rayIn: &Ray, hitRecord: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = *hitRecord.intersection() + *hitRecord.normal() + randomInUnitSphere();
        let scattered = Ray::new(
            *hitRecord.intersection(),
            target - *hitRecord.intersection(),
        );
        let attenuation = self.albedo.value(hitRecord.uv(), hitRecord.intersection());
        return Some((scattered, attenuation));
    }
}

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Arc<dyn Texture>,
    fuzziness: f64,
}

impl Metal {
    pub fn new<T>(albedo: T, fuzziness: f64) -> Self
    where
        T: Into<Arc<dyn Texture>>,
    {
        Self {
            albedo: albedo.into(),
            fuzziness: fuzziness,
        }
    }

    pub fn albedo(&self) -> &Arc<dyn Texture> {
        return &self.albedo;
    }

    pub fn fuzziness(&self) -> f64 {
        return self.fuzziness;
    }
}

impl Material for Metal {
    fn scatter(&self, rayIn: &Ray, hitRecord: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = rayIn.direction().normalized().reflected(hitRecord.normal());
        let scattered = Ray::new(
            *hitRecord.intersection(),
            if self.fuzziness == 0.0 {
                reflected
            } else {
                reflected + self.fuzziness * randomInUnitSphere()
            },
        );
        let attenuation = self.albedo.value(hitRecord.uv(), hitRecord.intersection());
        if rayIn.direction().dot(hitRecord.normal()) < 0.0 {
            return Some((scattered, attenuation));
        } else {
            // eprintln!("{:#?} {:#?} {:#?}", rayIn, hitRecord.normal(), scattered);
            // 我发现反射居然有的时候也会内表面反射，很奇怪
            // 破案了，是浮点数精度问题。入射光反射的时候，因为精度不够，算出来的反射点坐标有时候会偏移到球的内部
            return None;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dielectric {
    // 折射物质没有texture？
    refractive: f64,
}

impl Dielectric {
    pub fn new(refractive: f64) -> Self {
        Self {
            refractive: refractive,
        }
    }

    pub fn refractive(&self) -> f64 {
        return self.refractive;
    }
}

impl Material for Dielectric {
    fn scatter(&self, rayIn: &Ray, hitRecord: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refractiveInOverOut = 1.0;
        let mut normal = *hitRecord.normal();

        if rayIn.direction().dot(hitRecord.normal()) < 0.0 {
            refractiveInOverOut = 1.0 / self.refractive;
        } else {
            refractiveInOverOut = self.refractive;
            normal = -normal;
        }

        if let Some(refracted) = rayIn
            .direction()
            .normalized()
            .refracted(&normal, refractiveInOverOut)
        {
            // 折射
            return Some((Ray::new(*hitRecord.intersection(), refracted), attenuation));
        } else {
            // 全反射
            return Some((
                Ray::new(
                    *hitRecord.intersection(),
                    rayIn.direction().normalized().reflected(&normal),
                ),
                attenuation,
            ));
            // 但是图上有明显的一圈一圈的杂质，不知道是什么原因
            // 破案了，是浮点数精度的那个问题。解决了浮点数精度问题就好了
        }
    }
}

// 纹理，直接按uv和空间坐标返回颜色
pub trait Texture: Send + Sync + Debug {
    fn value(&self, uv: &(f64, f64), point: &Vec3) -> Vec3;
}

#[derive(Clone, Debug)]
pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> Self {
        Self { color: color }
    }
}

impl Texture for SolidColor {
    fn value(&self, uv: &(f64, f64), point: &Vec3) -> Vec3 {
        return self.color.clone();
    }
}

#[derive(Clone, Debug)]
pub struct CheckerTexture {
    black: Arc<dyn Texture>,
    white: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new<T>(black: T, white: T) -> Self
    where
        T: Into<Arc<dyn Texture>>,
    {
        Self {
            black: black.into(),
            white: white.into(),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: &(f64, f64), point: &Vec3) -> Vec3 {
        // let sine = (10.0 * point.x()).sin() * (10.0 * point.y()).sin() * (10.0 * point.z()).sin(); // 直接按空间绝对坐标来决定用black还是white材质有点奇怪
        let sine = (2.0 * PI * 10.0 * uv.0).sin() * (2.0 * PI * 10.0 * uv.1).sin(); // 频率100.0 m^{-1}，也就是每米有100个黑白四方格单位
        if sine > 0.0 {
            return self.black.value(uv, point);
        } else {
            return self.white.value(uv, point);
        }
    }
}

pub struct ImageTexture<T> {
    mapping: T,
}

impl<T> ImageTexture<T> {
    pub fn new(mapping: T) -> Self {
        Self { mapping: mapping }
    }
}

// 不知道怎么存图片数据，让外层处理吧
impl<T> Texture for ImageTexture<T>
where
    T: (Fn(&(f64, f64)) -> Vec3) + Send + Sync,
{
    fn value(&self, uv: &(f64, f64), point: &Vec3) -> Vec3 {
        return (self.mapping)(uv);
    }
}

impl<T> Debug for ImageTexture<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ImageTexture")
    }
}

// 发光材质
#[derive(Debug, Clone)]
pub struct DiffuseLight {
    emission: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new<T>(emission: T) -> Self
    where
        T: Into<Arc<dyn Texture>>,
    {
        Self {
            emission: emission.into(),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, rayIn: &Ray, hitRecord: &HitRecord) -> Option<(Ray, Vec3)> {
        return None;
    }

    fn emitted(&self, uv: &(f64, f64), point: &Vec3) -> Vec3 {
        return self.emission.value(uv, point);
    }
}

// 各项同性随机散射
#[derive(Debug, Clone)]
pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new<T>(albedo: T) -> Self
    where
        T: Into<Arc<dyn Texture>>,
    {
        Self {
            albedo: albedo.into(),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, rayIn: &Ray, hitRecord: &HitRecord) -> Option<(Ray, Vec3)> {
        let scattered = Ray::new(hitRecord.intersection().clone(), randomInUnitSphere());
        let attenuation = self.albedo.value(hitRecord.uv(), hitRecord.intersection());
        return Some((scattered, attenuation));
    }
}
