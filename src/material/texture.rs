mod perlin;
mod rtw_stb_image;

use crate::material::texture::rtw_stb_image::RtwImage;
use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::Point3;
use perlin::Perlin;
use std::rc::Rc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
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
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub(crate) fn new_color(scale: f64, c1: &Color, c2: &Color) -> Self {
        Self::new(
            scale,
            Rc::new(SolidColor::new(c1)),
            Rc::new(SolidColor::new(c2)),
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
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(self.scale * *p)))
    }
}
