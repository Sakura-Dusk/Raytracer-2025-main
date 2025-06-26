use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use crate::{hittable};
use crate::hittable::Hittable;
use crate::rtweekend::{color};
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::{unit_vector, Point3, Vec3};
use crate::rtweekend::vec3::ray::Ray;

pub(crate) struct Camera {
    pub aspect_ratio:f64,//default in 1.0
    pub image_width:u32,//default in 100

    image_height:u32,
    center:Point3,
    pixel00_loc:Point3,
    pixel_delta_u:Vec3,
    pixel_delta_v:Vec3,
}

impl Camera {
    pub(crate) fn new() -> Camera {
        Camera {
            aspect_ratio:1.0,
            image_width:100,
            image_height:0,
            center:Point3::new(0.0,0.0,0.0),
            pixel00_loc:Point3::new(0.0,0.0,0.0),
            pixel_delta_u:Vec3::new(0.0,0.0,0.0),
            pixel_delta_v:Vec3::new(0.0,0.0,0.0),
        }
    }
}

impl Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };
        
        self.center = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        
        let view_point_height = 2.0;
        let view_point_width = view_point_height * (self.image_width as f64 / self.image_height as f64);
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

    fn ray_color(r: &Ray, world: &dyn Hittable) -> color::Color {
        let mut rec: hittable::HitRecord = hittable::HitRecord::new();
        if world.hit(&r, &Interval::new(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + color::Color::new(1.0, 1.0, 1.0));
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

    pub fn render(&mut self, world:&dyn Hittable) {
        self.initialize();
        
        let path = std::path::Path::new("output/book1/image5.png");
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

                let pixel_center =
                    self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = Camera::ray_color(&r, world);
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