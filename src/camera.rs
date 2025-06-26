use crate::hittable::Hittable;
use crate::rtweekend::color;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{Point3, Vec3, random_on_hemisphere, unit_vector};
use crate::{hittable, rtweekend};
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub(crate) struct Camera {
    pub aspect_ratio: f64,      //default in 1.0
    pub image_width: u32,       //default in 100
    pub samples_per_pixel: u32, //default in 10

    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub(crate) fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let view_point_height = 2.0;
        let view_point_width =
            view_point_height * (self.image_width as f64 / self.image_height as f64);
        let focal_length = 1.0; //the distance between camera and item

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

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left = self.center
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            }
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(
            rtweekend::random_double() - 0.5,
            rtweekend::random_double() - 0.5,
            0.0,
        )
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> color::Color {
        let mut rec: hittable::HitRecord = hittable::HitRecord::new();
        if world.hit(&r, &Interval::new(0.0, f64::INFINITY), &mut rec) {
            let direction = random_on_hemisphere(&rec.normal);
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), world);
        }

        let unit_direction = unit_vector(&r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a)
            * color::Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            + a * color::Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        let path = std::path::Path::new("output/book1/image7.png");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

        // different from the book, we use image crate to create a .png image rather than outputting .ppm file, which is not widely used.
        // anyway, you may output any image format you like.
        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        //Render
        for j in (0..self.image_height).rev() {
            for i in 0..self.image_width {
                let pixel = img.get_pixel_mut(i, j);

                let mut pixel_color = color::Color::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world);
                }
                pixel_color = pixel_color * self.pixel_samples_scale;
                color::write_color(pixel, &pixel_color);
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
}
