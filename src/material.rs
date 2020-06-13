use crate::ray::HitRecord;
use crate::ray::Ray;
use crate::util::randomInUnitSphere;
use crate::vec3::Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, rayIn: &Ray, hitRecord: &HitRecord) -> Option<(Ray, Vec3)>;
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
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo: albedo }
    }

    pub fn albedo(&self) -> &Vec3 {
        return &self.albedo;
    }
}

impl Material for Metal {
    fn scatter(&self, rayIn: &Ray, hitRecord: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = rayIn.direction().normalized().reflected(hitRecord.normal());
        let scattered = Ray::new(*hitRecord.intersection(), reflected);
        let attenuation = self.albedo;
        if scattered.direction().dot(hitRecord.normal()) > 0.0 {
            return Some((scattered, attenuation));
        } else {
            return None;
        }
    }
}
