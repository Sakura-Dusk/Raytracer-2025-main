use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::Point3;
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
