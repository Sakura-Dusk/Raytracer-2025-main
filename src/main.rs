mod vec3;

use console::style;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;

fn main() {
    let path = std::path::Path::new("output/book1/image1.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let width = 256;
    let height = 256;
    // different from the book, we use image crate to create a .png image rather than outputting .ppm file, which is not widely used.
    // anyway, you may output any image format you like.
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);
            let pixel_color = Color {
                x : (i as f64) / ((width - 1) as f64),
                y : (j as f64) / ((height - 1) as f64),
                z : 0.0,
            };
            write_color(pixel, &pixel_color);
            // let r: f64 = (i as f64) / ((width - 1) as f64) * 255.999;
            // let g: f64 = (j as f64) / ((height - 1) as f64) * 255.999;
            // let b: f64 = 0.0 * 255.999;
            // *pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
        progress.inc(1);
    }
    progress.finish();

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    img.save(path).expect("Cannot save the image to the file");
}

type Color = vec3::Vec3;
fn write_color(pixel : &mut Rgb<u8>, pixel_color : &Color) {
    let r : f64 = pixel_color.x * 255.999;
    let g : f64 = pixel_color.y * 255.999;
    let b : f64 = pixel_color.z * 255.999;
    *pixel = Rgb([r as u8,g as u8,b as u8]);
}