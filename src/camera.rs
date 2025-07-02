use crate::material::hittable;
use crate::material::hittable::Hittable;
use crate::rtweekend::color::Color;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{Point3, Vec3, random_in_unit_disk, unit_vector};
use crate::rtweekend::{PI, color, degrees_to_radians, random_double, vec3};
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rayon::prelude::*;

pub(crate) struct Camera {
    pub aspect_ratio: f64,      //default in 1.0
    pub image_width: u32,       //default in 100
    pub samples_per_pixel: u32, //default in 10
    pub max_depth: i32,         // default in 10
    pub background: Color,

    pub vfov: f64, // Vertical view angle (field of view)
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: u32,
    pixel_samples_scale: f64,
    sqrt_spp: i32,
    recip_sqrt_spp: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3, //Camera frame basis vectors
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub(crate) fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            background: Color::new(0.0, 0.0, 0.0),

            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_dist: 1.0,

            image_height: 0,
            pixel_samples_scale: 0.0,
            sqrt_spp: 1,
            recip_sqrt_spp: 1.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(1.0, 0.0, 0.0),
            v: Vec3::new(0.0, 1.0, 0.0),
            w: Vec3::new(0.0, 0.0, 1.0),
            defocus_disk_u: Vec3::new(1.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 1.0, 0.0),
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

        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as i32;
        self.pixel_samples_scale = 1.0 / (self.sqrt_spp * self.sqrt_spp) as f64;
        self.recip_sqrt_spp = 1.0 / self.sqrt_spp as f64;

        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = unit_vector(&(self.lookfrom - self.lookat));
        self.u = unit_vector(&vec3::cross(&self.vup, &self.w));
        self.v = vec3::cross(&self.w, &self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn sample_square_stratified(&self, s_i: u32, s_j: u32) -> Vec3 {
        let px = ((s_i as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;

        Vec3::new(px, py, 0.0)
    }
    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn get_ray(&self, i: u32, j: u32, s_i: u32, s_j: u32) -> Ray {
        // let offset = self.sample_square_stratified(s_i, s_j);
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::new_move(ray_origin, ray_direction, ray_time)
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec: hittable::HitRecord = hittable::HitRecord::new();
        if !world.hit(&r, &mut Interval::new(0.001, f64::INFINITY), &mut rec) {
            return self.background.clone();
        }

        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);

        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }

        let scattering_pdf = rec.mat.scattering_pdf(r, &rec, &mut scattered);
        // let pdf_value = 1.0 / (2.0 * PI);
        let pdf_value = scattering_pdf;
        
        let color_from_scatter =
            (attenuation * scattering_pdf * self.ray_color(&scattered, depth - 1, world))
                / pdf_value;

        color_from_emission + color_from_scatter
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        let path = std::path::Path::new("output/book3/image3.png");
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

        let pixels: Vec<_> = (0..self.image_height)
            .rev()
            .flat_map(|j| (0..self.image_width).map(move |i| (i, j)))
            .collect();

        let colors: Vec<Color> = pixels
            .par_iter() // 现在可以正确调用
            .map(|&(i, j)| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for s_j in 0..self.sqrt_spp {
                    for s_i in 0..self.sqrt_spp {
                        let r = self.get_ray(i, j, s_i as u32, s_j as u32);
                        pixel_color += self.ray_color(&r, self.max_depth, world);
                    }
                }
                pixel_color * self.pixel_samples_scale
            })
            .collect();

        for ((i, j), color) in pixels.into_iter().zip(colors) {
            let pixel = img.get_pixel_mut(i, j);
            color::write_color(pixel, &color);
            progress.inc(1);
        }

        // //Render
        // for j in (0..self.image_height).rev() {
        //     for i in 0..self.image_width {
        //         let pixel = img.get_pixel_mut(i, j);
        //
        //         let mut pixel_color = color::Color::new(0.0, 0.0, 0.0);
        //         for _ in 0..self.samples_per_pixel {
        //             let r = self.get_ray(i, j);
        //             let rate = 0.5;
        //             pixel_color += self.ray_color(&r, self.max_depth, world, rate);
        //         }
        //         pixel_color = pixel_color * self.pixel_samples_scale;
        //         color::write_color(pixel, &pixel_color);
        //         progress.inc(1);
        //     }
        // }
        progress.finish();

        println!(
            "Output image as \"{}\"",
            style(path.to_str().unwrap()).yellow()
        );
        img.save(path).expect("Cannot save the image to the file");
    }
}
