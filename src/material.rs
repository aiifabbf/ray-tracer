use crate::ray::HitRecord;
use crate::ray::Ray;
use crate::util::randomInUnitSphere;
use crate::vec3::Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, rayIn: &Ray, hitRecord: &HitRecord) -> Option<(Ray, Vec3)>;
    // 我觉得这里有点问题，真实世界里一束入射光会散射出多束反射光，但是这里只会返回一束光，以后怎么扩展成多束光呢
}

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo: albedo }
    }

    pub fn albedo(&self) -> &Vec3 {
        return &self.albedo;
    }
}

impl Material for Lambertian {
    fn scatter(&self, rayIn: &Ray, hitRecord: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = *hitRecord.intersection() + *hitRecord.normal() + randomInUnitSphere();
        let scattered = Ray::new(
            *hitRecord.intersection(),
            target - *hitRecord.intersection(),
        );
        let attenuation = self.albedo;
        return Some((scattered, attenuation));
    }
}

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Self {
        Self {
            albedo: albedo,
            fuzziness: fuzziness,
        }
    }

    pub fn albedo(&self) -> &Vec3 {
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
        let attenuation = self.albedo;
        if hitRecord.front() {
            return Some((scattered, attenuation));
        } else {
            // eprintln!("1");
            // 我发现反射居然有的时候也会内表面反射，很奇怪
            return None;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dielectric {
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

        if hitRecord.front() {
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
        }
    }
}
