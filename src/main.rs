mod camera;
mod material;
mod rtweekend;

use crate::camera::Camera;
use crate::material::hittable::bvh::BvhNode;
use crate::material::hittable::constant_medium::ConstantMedium;
use crate::material::hittable::hittable_list::HittableList;
use crate::material::hittable::quad::{Quad, make_box};
use crate::material::hittable::sphere::Sphere;
use crate::material::hittable::{Hittable, RotateY, Translate};
use crate::material::texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::rtweekend::color::Color;
use crate::rtweekend::random_double;
use crate::rtweekend::random_double_range;
use crate::rtweekend::vec3::Point3;
use rtweekend::vec3::Vec3;
use std::rc::Rc;

fn main() {
    let opt = 10;
    match opt {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        _ => final_scene(400, 250, 4),
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

fn cornell_box() {
    let mut world: HittableList = HittableList::new();

    let red = Rc::new(Lambertian::new(&Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(&Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new_color(&Color::new(15.0, 15.0, 15.0)));

    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Point3::new(-130.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -130.0),
        light.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Point3::new(-555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1: Rc<dyn Hittable> = make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: Rc<dyn Hittable> = make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn cornell_smoke() {
    let mut world = HittableList::new();

    let red = Rc::new(Lambertian::new(&Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(&Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new_color(&Color::new(7.0, 7.0, 7.0)));

    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Point3::new(330.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 305.0),
        light.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1: Rc<dyn Hittable> = make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Rc<dyn Hittable> = make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Rc::new(ConstantMedium::new_color(
        box1,
        0.01,
        &Color::new(0.0, 0.0, 0.0),
    )));
    world.add(Rc::new(ConstantMedium::new_color(
        box2,
        0.01,
        &Color::new(1.0, 1.0, 1.0),
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: i32) {
    let mut boxes1 = HittableList::new();
    let ground = Rc::new(Lambertian::new(&Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(make_box(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();

    world.add(Rc::new(BvhNode::new(boxes1)));

    let light = Rc::new(DiffuseLight::new_color(&Color::new(7.0, 7.0, 7.0)));
    world.add(Rc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light.clone(),
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Rc::new(Lambertian::new(&Color::new(0.7, 0.3, 0.1)));
    world.add(Rc::new(Sphere::new_move(
        center1,
        center2,
        50.0,
        sphere_material.clone(),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(&Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundary = Rc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Rc::new(ConstantMedium::new_color(
        boundary.clone(),
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    )));
    boundary = Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(Rc::new(ConstantMedium::new_color(
        boundary.clone(),
        0.0001,
        &Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Rc::new(Lambertian::new_tex(Rc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    world.add(Rc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat.clone(),
    )));
    let pertext = Rc::new(NoiseTexture::new(0.2));
    world.add(Rc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Rc::new(Lambertian::new_tex(pertext.clone())),
    )));

    let mut boxes2 = HittableList::new();
    let white = Rc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for j in 0..ns {
        boxes2.add(Rc::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(Rc::new(BvhNode::new(boxes2)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}
