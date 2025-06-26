use crate::rtweekend::vec3::Vec3;
use image::Rgb;

pub(crate) type Color = Vec3;
pub(crate) fn write_color(pixel: &mut Rgb<u8>, pixel_color: &Color) {
    let r: f64 = pixel_color.x * 255.999;
    let g: f64 = pixel_color.y * 255.999;
    let b: f64 = pixel_color.z * 255.999;
    *pixel = Rgb([r as u8, g as u8, b as u8]);
}
