use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::Vec3;
use image::Rgb;

pub(crate) type Color = Vec3;
pub(crate) fn write_color(pixel: &mut Rgb<u8>, pixel_color: &Color) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    //Translate the [0,1] component values to the byte range [0,255].
    let intensity: Interval = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as u8;
    let gbyte = (256.0 * intensity.clamp(g)) as u8;
    let bbyte = (256.0 * intensity.clamp(b)) as u8;

    *pixel = Rgb([rbyte, gbyte, bbyte]);
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}
