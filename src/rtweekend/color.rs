use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::Vec3;
use image::Rgb;

pub(crate) type Color = Vec3;
pub(crate) fn write_color(pixel: &mut Rgb<u8>, pixel_color: &Color) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    //Translate the [0,1] component values to the byte range [0,255].
    let intensity: Interval = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as u8;
    let gbyte = (256.0 * intensity.clamp(g)) as u8;
    let bbyte = (256.0 * intensity.clamp(b)) as u8;

    *pixel = Rgb([rbyte, gbyte, bbyte]);
}
