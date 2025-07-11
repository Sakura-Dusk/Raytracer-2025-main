use crate::material::hittable::HitRecord;
use crate::material::texture::SolidColor;
use crate::material::texture::Texture;
use crate::material::texture::rtw_stb_image::RtwImage;
use crate::pdf::{CosinePdf, Pdf, SpherePdf};
use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{
    Point3, Vec3, dot, random_unit_vector, reflect, refract, unit_vector,
};
use crate::rtweekend::{PI, random_double, vec3};
use std::sync::Arc;

pub mod hittable;
pub(crate) mod onb;
pub(crate) mod texture;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Arc<dyn Pdf>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl ScatterRecord {
    pub fn default() -> ScatterRecord {
        Self {
            attenuation: Color::default(),
            pdf_ptr: Arc::new(SpherePdf::default()),
            skip_pdf: bool::default(),
            skip_pdf_ray: Ray::default(),
        }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        false
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(0.00, 0.00, 0.00)
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }

    fn check_normal_mapping(&self) -> bool {
        false
    }

    fn get_normal_mapping(&self, u: f64, v: f64) -> Vec3 {
        Vec3::default()
    }

    fn check_alpha_mapping(&self) -> bool {
        false
    }

    fn get_alpha_mapping(&self, u: f64, v: f64) -> f64 {
        0.0
    }

    fn check_light_mapping(&self) -> bool {
        false
    }

    fn get_light_mapping(&self, u: f64, v: f64) -> Color {
        Vec3::default()
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Arc::new(CosinePdf::new(&rec.normal));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = dot(&rec.normal, &unit_vector(&scattered.direction));
        if cos_theta < 0.0 { 0.0 } else { cos_theta / PI }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let reflected = reflect(&r_in.direction, &rec.normal);
        let reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());

        srec.attenuation = self.albedo.clone();
        srec.pdf_ptr = Arc::new(SpherePdf::default());
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::new_move(rec.p, reflected, r_in.time);

        true
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        srec.pdf_ptr = Arc::new(SpherePdf::default());
        srec.skip_pdf = true;
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

        srec.skip_pdf_ray = Ray::new_move(rec.p, direction, r_in.time);
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
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !rec.front_face {
            return Color::new(0.00, 0.00, 0.00);
        }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Arc::new(SpherePdf::default());
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}

pub struct Mapping {
    basis_material: Arc<dyn Material>,
    pub normal_mapping: Option<RtwImage>,
    pub alpha_mapping: Option<RtwImage>,
    pub light_mapping: Option<RtwImage>,
}

impl Mapping {
    pub(crate) fn default() -> Self {
        Self {
            basis_material: Arc::new(Metal::new(&Color::default(), 0.0)),
            normal_mapping: None,
            alpha_mapping: None,
            light_mapping: None,
        }
    }

    pub(crate) fn new(basis_material: Arc<dyn Material>) -> Self {
        Self {
            basis_material,
            normal_mapping: None,
            alpha_mapping: None,
            light_mapping: None,
        }
    }

    pub fn set_normal_mapping(&mut self, normal_mapping: RtwImage) {
        self.normal_mapping = Some(normal_mapping);
    }

    pub fn set_alpha_mapping(&mut self, alpha_mapping: RtwImage) {
        self.alpha_mapping = Some(alpha_mapping);
    }

    pub fn set_light_mapping(&mut self, light_mapping: RtwImage) {
        self.light_mapping = Some(light_mapping);
    }
}

impl Material for Mapping {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        self.basis_material.scatter(r_in, rec, srec)
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !rec.front_face {
            return Color::new(0.0, 0.0, 0.0);
        }
        if !self.check_light_mapping() {
            return self.basis_material.emitted(r_in, rec, u, v, p);
        }
        self.get_light_mapping(u, v)
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        self.basis_material.scattering_pdf(r_in, rec, scattered)
    }

    fn check_normal_mapping(&self) -> bool {
        if self.normal_mapping.is_some() {
            true
        } else {
            false
        }
    }

    fn get_normal_mapping(&self, u: f64, v: f64) -> Vec3 {
        let mp = self.normal_mapping.as_ref().unwrap();
        let col = mp.pixel_data(
            (u * mp.width() as f64) as usize,
            (v * mp.height() as f64) as usize,
        );
        // println!("{} {}", u, v);
        // println!("{} {} {}", col[0], col[1], col[2]);
        Vec3::new(
            col[0] as f64 / 256.0 * 2.0 - 1.0,
            col[1] as f64 / 256.0 * 2.0 - 1.0,
            col[2] as f64 / 256.0 * 2.0 - 1.0,
        )
    }

    fn check_alpha_mapping(&self) -> bool {
        if self.alpha_mapping.is_some() {
            true
        } else {
            false
        }
    }

    fn get_alpha_mapping(&self, u: f64, v: f64) -> f64 {
        let mp = self.alpha_mapping.as_ref().unwrap();
        let col = mp.pixel_data(
            (u * mp.width() as f64) as usize,
            (v * mp.height() as f64) as usize,
        );
        // println!("{} {}", u, v);
        // println!("{} {} {}", col[0], col[1], col[2]);
        col[0] as f64 / 256.0
    }

    fn check_light_mapping(&self) -> bool {
        if self.light_mapping.is_some() {
            true
        } else {
            false
        }
    }

    fn get_light_mapping(&self, u: f64, v: f64) -> Color {
        let mp = self.light_mapping.as_ref().unwrap();

        if mp.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * mp.width() as f64) as usize;
        let j = (v * mp.height() as f64) as usize;
        let pixel = mp.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(
            (color_scale * pixel[0] as f64) * (color_scale * pixel[0] as f64),
            (color_scale * pixel[1] as f64) * (color_scale * pixel[1] as f64),
            (color_scale * pixel[2] as f64) * (color_scale * pixel[2] as f64),
        )
    }
}
