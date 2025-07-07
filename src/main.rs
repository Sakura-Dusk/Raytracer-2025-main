mod camera;
mod material;
mod pdf;
mod rtweekend;

use crate::camera::Camera;
use crate::material::hittable::hittable_list::HittableList;
use crate::material::hittable::quad::{Quad, make_box};
use crate::material::hittable::sphere::Sphere;
use crate::material::hittable::triangle::Triangle;
use crate::material::hittable::{Hittable, RotateY, Translate};
use crate::material::texture::model::get_models;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::Point3;
use rtweekend::vec3::Vec3;
use std::sync::Arc;
use std::time::Instant;
use tobj::Model;

fn main() {
    let start = Instant::now();

    try_use_model();

    let duration = start.elapsed();
    println!("耗时: {:.2}秒", duration.as_secs_f64());
}
fn cornell_box() {
    let mut world: HittableList = HittableList::new();

    let red = Arc::new(Lambertian::new(&Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(&Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(&Color::new(7.0, 7.0, 7.0)));

    let mirror = Arc::new(Metal::new(&Color::new(0.8, 1.0, 0.9), 0.0));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        Point3::new(0.0, 555.0, 0.0),
        green.clone(),
    )));
    // world.add(Arc::new(Triangle::new(
    //     Point3::new(555.0, 555.0, 555.0),
    //     Point3::new(0.0, 0.0, -555.0),
    //     Point3::new(0.0, -555.0, 0.0),
    //     mirror.clone(),
    // )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Point3::new(0.0, 0.0, -555.0),
        Point3::new(0.0, 555.0, 0.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 555.0),
        Point3::new(-555.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    world.add(Arc::new(Triangle::new(
        Point3::new(213.0, 554.0, 227.0),
        Point3::new(130.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 105.0),
        light.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass.clone(),
    )));

    let empty_material: Arc<dyn Material> = Arc::new(Lambertian::new(&Color::new(0.0, 0.0, 0.0)));
    let mut lights = HittableList::new();
    lights.add(Arc::new(Triangle::new(
        Point3::new(213.0, 554.0, 227.0),
        Point3::new(130.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 105.0),
        empty_material.clone(),
    )));
    lights.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        empty_material.clone(),
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 1000;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world, Arc::new(lights));
}

fn try_use_model() {
    let mut world: HittableList = HittableList::new();

    let model = get_models("cornell_box.obj", 1.0);
    world.add(model);
    let model = get_models("miku/miku01.obj", 0.2);
    let model = Arc::new(Translate::new(model, Vec3::new(200.0, 165.5, 200.0)));
    world.add(model);

    let light = Arc::new(DiffuseLight::new_color(&Color::new(15.0, 15.0, 15.0)));
    let light1 = Arc::new(DiffuseLight::new_color(&Color::new(30.0, 30.0, 30.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(213.0, 548.799, 127.0),
        Point3::new(130.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 105.0),
        light.clone(),
    )));
    let feet_light = Arc::new(Quad::new(
        Point3::new(555.0, 0.2, 0.0),
        Point3::new(-50.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 50.0),
        light1.clone(),
    ));
    let feet_light = Arc::new(Translate::new(feet_light, Vec3::new(-320.0, 165.0, 120.0)));
    world.add(feet_light);

    let empty_material: Arc<dyn Material> = Arc::new(Lambertian::new(&Color::new(0.0, 0.0, 0.0)));
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        Point3::new(213.0, 548.799, 127.0),
        Point3::new(130.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 105.0),
        empty_material.clone(),
    )));
    lights.add(Arc::new(Quad::new(
        Point3::new(555.0 - 320.0, 0.2 + 165.0, 0.0 + 120.0),
        Point3::new(-50.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 50.0),
        empty_material.clone(),
    )));

    // let empty_material: Arc<dyn Material> = Arc::new(Lambertian::new(&Color::new(0.0, 0.0, 0.0)));
    // let mut lights = HittableList::new();
    // lights.add(Arc::new(Quad::new(
    //     Point3::new(213.0, 554.0, 227.0),
    //     Point3::new(130.0, 0.0, 0.0),
    //     Point3::new(0.0, 0.0, 105.0),
    //     empty_material.clone(),
    // )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 1000;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world, Arc::new(lights));
}
