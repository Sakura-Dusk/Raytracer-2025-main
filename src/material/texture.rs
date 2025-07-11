pub(crate) mod model;
mod mtl;
mod perlin;
pub(crate) mod rtw_stb_image;

use crate::material::texture::rtw_stb_image::RtwImage;
use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::{Point3, Vec3};
use perlin::Perlin;
use std::ops::{Add, Mul, Sub};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct UV {
    pub u: f64,
    pub v: f64,
}
impl UV {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }
}

impl Add for UV {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            u: self.u + other.u,
            v: self.v + other.v,
        }
    }
}

impl Sub for UV {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            u: self.u - other.u,
            v: self.v - other.v,
        }
    }
}

impl Mul<f64> for UV {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            u: self.u * other,
            v: self.v * other,
        }
    }
}

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;

    fn normal(&self, u: f64, v: f64, normal: Vec3, tangent: Vec3, bitangent: Vec3) -> Option<Vec3> {
        None
    }

    fn alpha(&self, u: f64, v: f64) -> Option<f64> {
        None
    }
}

pub(crate) struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: &Color) -> Self {
        SolidColor { albedo: *albedo }
    }

    pub fn new_rgb(red: f64, green: f64, blue: f64) -> Self {
        SolidColor {
            albedo: Color::new(red, green, blue),
        }
    }
}
impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.albedo
    }
}

pub(crate) struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub(crate) fn new_color(scale: f64, c1: &Color, c2: &Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(c1)),
            Arc::new(SolidColor::new(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = (self.inv_scale * p.x).floor() as i32;
        let y_integer = (self.inv_scale * p.y).floor() as i32;
        let z_integer = (self.inv_scale * p.z).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: RtwImage::new(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.height() as f64) as usize;
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(
            (color_scale * pixel[0] as f64) * (color_scale * pixel[0] as f64),
            (color_scale * pixel[1] as f64) * (color_scale * pixel[1] as f64),
            (color_scale * pixel[2] as f64) * (color_scale * pixel[2] as f64),
        )
    }
}

#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::default(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(*p, 7)).sin())
    }
}

pub struct Orthogonal {
    cols: [Vec3; 3],
}

impl Orthogonal {
    pub fn from_cols(t: Vec3, b: Vec3, n: Vec3) -> Self {
        Self { cols: [t, b, n] }
    }

    pub fn mul_vec3(&self, v: Vec3) -> Vec3 {
        self.cols[0] * v.x + self.cols[1] * v.y + self.cols[2] * v.z
    }
}

struct MappedTexture {
    color_map: RtwImage,
    normal_map: Option<RtwImage>,
    alpha_map: Option<RtwImage>,
}

impl MappedTexture {
    pub fn new(path: &str, normal_path: Option<&str>, alpha_path: Option<&str>) -> Self {
        Self {
            color_map: RtwImage::new(path),
            normal_map: normal_path.map(RtwImage::new),
            alpha_map: alpha_path.map(RtwImage::new),
        }
    }
}

impl Texture for MappedTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        if self.color_map.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.color_map.width() as f64) as usize;
        let j = (v * self.color_map.height() as f64) as usize;
        let pixel = self.color_map.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(
            (color_scale * pixel[0] as f64) * (color_scale * pixel[0] as f64),
            (color_scale * pixel[1] as f64) * (color_scale * pixel[1] as f64),
            (color_scale * pixel[2] as f64) * (color_scale * pixel[2] as f64),
        )
    }

    fn normal(&self, u: f64, v: f64, normal: Vec3, tangent: Vec3, bitangent: Vec3) -> Option<Vec3> {
        let map = self.normal_map.as_ref()?;
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * map.width() as f64) as usize;
        let j = (v * map.height() as f64) as usize;
        let pixel = map.pixel_data(i, j);

        let nx = 2.0 * (pixel[0] as f64 / 255.0) - 1.0;
        let ny = 2.0 * (pixel[1] as f64 / 255.0) - 1.0;
        let nz = 2.0 * (pixel[2] as f64 / 255.0) - 1.0;

        let tangent_space_normal = Vec3::new(nx, ny, nz);
        let tbn = Orthogonal::from_cols(tangent, bitangent, normal);
        Some(tbn.mul_vec3(tangent_space_normal))
    }

    fn alpha(&self, u: f64, v: f64) -> Option<f64> {
        let map = self.alpha_map.as_ref()?;
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * map.width() as f64) as usize;
        let j = (v * map.height() as f64) as usize;
        let pixel = map.pixel_data(i, j);

        Some(pixel[0] as f64 / 255.0)
    }
}
