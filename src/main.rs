mod camera;
mod material;
mod rtweekend;

use crate::camera::Camera;
use crate::material::hittable::hittable_list::HittableList;
use crate::material::hittable::quad::Quad;
use crate::material::hittable::sphere::Sphere;
use crate::material::texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::rtweekend::color::Color;
use crate::rtweekend::random_double;
use crate::rtweekend::random_double_range;
use crate::rtweekend::vec3::Point3;
use rtweekend::vec3::Vec3;
use std::rc::Rc;

fn main() {
    let opt = 6;
    match opt {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        _ => (),
    }
}
fn bouncing_spheres() {
    //World build
    let mut world: HittableList = HittableList::new();

    let checker = Rc::new(CheckerTexture::new_color(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_tex(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random() * Color::random();
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    let sphere_material: Rc<dyn Material> = Rc::new(Lambertian::new(&albedo));
                    world.add(Rc::new(Sphere::new_move(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material: Rc<dyn Material> = Rc::new(Metal::new(&albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    //glass
                    let sphere_material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Rc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Rc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let world = material::hittable::bvh::BvhNode::new(world);

    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}

fn checkered_spheres() {
    let mut world: HittableList = HittableList::new();

    let checker = Rc::new(CheckerTexture::new_color(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new_tex(checker.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new_tex(checker.clone())),
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn earth() {
    let earth_texture: Rc<dyn Texture> = Rc::new(ImageTexture::new("earthmap.jpg"));
    let erath_surface: Rc<dyn Material> = Rc::new(Lambertian::new_tex(Rc::clone(&earth_texture)));
    let globe = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, erath_surface));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let mut world: HittableList = HittableList::new();
    world.add(globe);

    cam.render(&world);
}

fn perlin_spheres() {
    let mut world: HittableList = HittableList::new();

    let pertext: Rc<dyn Texture> = Rc::new(NoiseTexture::new(4.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000., 0.0),
        1000.0,
        Rc::new(Lambertian::new_tex(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new_tex(pertext.clone())),
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn quads() {
    let mut world: HittableList = HittableList::new();

    //Materials
    let left_red = Rc::new(Lambertian::new(&Color::new(1.0, 0.2, 0.2)));
    let back_green = Rc::new(Lambertian::new(&Color::new(0.2, 1.0, 0.2)));
    let right_blue = Rc::new(Lambertian::new(&Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::new(&Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(Lambertian::new(&Color::new(0.2, 0.8, 0.8)));

    //Quads
    world.add(Rc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Point3::new(0.0, 0.0, -4.0),
        Point3::new(0.0, 4.0, 0.0),
        left_red.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Point3::new(4.0, 0.0, 0.0),
        Point3::new(0.0, 4.0, 0.0),
        back_green.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Point3::new(0.0, 0.0, 4.0),
        Point3::new(0.0, 4.0, 0.0),
        right_blue.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Point3::new(4.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 4.0),
        upper_orange.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Point3::new(4.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -4.0),
        lower_teal.clone(),
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn simple_light() {
    let mut world: HittableList = HittableList::new();

    let pretext = Rc::new(NoiseTexture::new(4.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_tex(pretext.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new_tex(pretext.clone())),
    )));

    let difflight = Rc::new(DiffuseLight::new_color(&Color::new(4.0, 4.0, 4.0)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.00, 0.00, 0.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
    cam.lookat = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}
