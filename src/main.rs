mod vec3;

use console::style;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;

use crate::vec3::ray::Ray;
use crate::vec3::{Vec3, unit_vector};

fn main() {
    let path = std::path::Path::new("output/book1/image4.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    // let width = 256;
    // let height = 256;
    let (image_width, image_height) = image_setup();
    // different from the book, we use image crate to create a .png image rather than outputting .ppm file, which is not widely used.
    // anyway, you may output any image format you like.
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    //set Camera
    let view_point_height = 2.0;
    let view_point_width = view_point_height * (image_width as f64 / image_height as f64);
    let focal_length = 1.0; //the distance between camera and item
    let camera_center = vec3::Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let viewport_u = Vec3 {
        x: view_point_width,
        y: 0.0,
        z: 0.0,
    };
    let viewport_v = Vec3 {
        x: 0.0,
        y: -view_point_height,
        z: 0.0,
    };

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    let viewport_upper_left = camera_center
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        }
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    //Render
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, j);

            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
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

type Color = Vec3;
fn write_color(pixel: &mut Rgb<u8>, pixel_color: &Color) {
    let r: f64 = pixel_color.x * 255.999;
    let g: f64 = pixel_color.y * 255.999;
    let b: f64 = pixel_color.z * 255.999;
    *pixel = Rgb([r as u8, g as u8, b as u8]);
}

fn image_setup() -> (u32, u32) {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    //Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    (image_width, image_height)
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(
        &vec3::Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
        r,
    );
    if t > 0.0 {
        let n = unit_vector(
            &(r.at(t)
                - Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                }),
        );
        return 0.5
            * Color {
                x: n.x + 1.0,
                y: n.y + 1.0,
                z: n.z + 1.0,
            };
    }
    let unit_direction = unit_vector(&r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a)
        * Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + a * Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
}

fn hit_sphere(center: &vec3::Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.origin;
    let a = vec3::dot(&r.direction, &r.direction);
    let b = -2.0 * vec3::dot(&r.direction, &oc);
    let c = vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
