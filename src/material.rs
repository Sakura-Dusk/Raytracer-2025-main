use crate::material::hittable::HitRecord;
use crate::material::texture::Texture;
use crate::material::texture::{CheckerTexture, SolidColor};
use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{dot, random_unit_vector, reflect, refract, unit_vector};
use crate::rtweekend::{random_double, vec3};
use std::rc::Rc;

pub mod hittable;
pub(crate) mod texture;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

impl dyn Material {
    pub(crate) fn new() {}
}

pub(crate) struct Lambertian {
    tex: Rc<dyn Texture>,
}

impl Lambertian {
    pub(crate) fn default() -> Lambertian {
        Lambertian {
            tex: Rc::new(SolidColor::new(&Color::new(0.5, 0.5, 0.5))),
        }
    }

    pub(crate) fn new(x: &Color) -> Lambertian {
        Lambertian {
            tex: Rc::new(SolidColor::new(&x)),
        }
    }

    pub(crate) fn new_tex(tex: Rc<dyn Texture>) -> Self {
        Self { tex: tex.clone() }
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

        *scattered = Ray::new_move(rec.p, scatter_direction, r_in.time);
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }
}

pub(crate) struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub(crate) fn default() -> Metal {
        Metal {
            albedo: Color::default(),
            fuzz: 0.0,
        }
    }

    pub(crate) fn new(x: &Color, fuzz: f64) -> Metal {
        Metal {
            albedo: x.clone(),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
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
        let reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new_move(rec.p, reflected, r_in.time);
        *attenuation = self.albedo.clone();
        dot(&scattered.direction, &rec.normal) > 0.0
    }
}

pub(crate) struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub(crate) fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(&r_in.direction);
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction: vec3::Vec3;

        if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_double() {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, ri);
        }

        *scattered = Ray::new_move(rec.p, direction, r_in.time);
        true
    }
}
