use crate::material::hittable::HitRecord;
use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{random_unit_vector, reflect};

pub mod hittable;
pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub(crate) struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub(crate) fn default() -> Lambertian {
        Lambertian {
            albedo: Color::default(),
        }
    }

    pub(crate) fn new(x: &Color) -> Lambertian {
        Lambertian { albedo: x.clone() }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo.clone();
        true
    }
}

pub(crate) struct Metal {
    albedo: Color,
}

impl Metal {
    pub(crate) fn default() -> Metal {
        Metal {
            albedo: Color::default(),
        }
    }

    pub(crate) fn new(x: &Color) -> Metal {
        Metal { albedo: x.clone() }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.direction, &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo.clone();
        true
    }
}
