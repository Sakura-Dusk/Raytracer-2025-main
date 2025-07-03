use crate::material::hittable::{HitRecord, Hittable};
use crate::material::onb::Onb;
use crate::material::texture::SolidColor;
use crate::material::texture::Texture;
use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{
    Point3, dot, random_cosine_direction, random_on_hemisphere, random_unit_vector, reflect,
    refract, unit_vector,
};
use crate::rtweekend::{PI, random_double, vec3};
use std::sync::Arc;

pub mod hittable;
mod onb;
pub(crate) mod texture;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }
}

impl dyn Material {
    pub(crate) fn new() {}
}

pub(crate) struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub(crate) fn default() -> Lambertian {
        Lambertian {
            tex: Arc::new(SolidColor::new(&Color::new(0.5, 0.5, 0.5))),
        }
    }

    pub(crate) fn new(x: &Color) -> Lambertian {
        Lambertian {
            tex: Arc::new(SolidColor::new(&x)),
        }
    }

    pub(crate) fn new_tex(tex: Arc<dyn Texture>) -> Self {
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
        pdf: &mut f64,
    ) -> bool {
        let uvw = Onb::new(&rec.normal);
        let mut scatter_direction = uvw.transform(&random_cosine_direction());

        *scattered = Ray::new_move(rec.p, unit_vector(&scatter_direction), r_in.time);
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        *pdf = dot(uvw.w(), &scattered.direction) / PI;
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        1.0 / (2.0 * PI)
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
        pdf: &mut f64,
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
        pdf: &mut f64,
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

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { tex }
    }

    pub fn new_color(emit: &Color) -> DiffuseLight {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}

struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(tex: Arc<dyn Texture>) -> Isotropic {
        Isotropic { tex }
    }

    pub fn new_color(albedo: &Color) -> Isotropic {
        Isotropic {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        *scattered = Ray::new_move(rec.p, random_unit_vector(), r_in.time);
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        *pdf = 1.0 / (4.0 * PI);
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
