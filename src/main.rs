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
use crate::material::texture::model::load_model;
use crate::material::texture::rtw_stb_image::RtwImage;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Mapping, Material, Metal};
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

    world.add(Arc::new(Quad::new(
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
    lights.add(Arc::new(Quad::new(
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

    // let model = get_models("cornell_box.obj", 1.0);
    // world.add(model);
    // load_model("cornell_box.obj", "cornell_box.mtl", &mut world, 0.0, Vec3::default());

    // load_model("miku/miku01.obj", "miku/miku01.mtl", &mut world, 0.0, Vec3::new(200.0, 165.5, 200.0), 0.2);

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2000.0,
        Arc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5))),
    )));

    let mut floor = Mapping::new(Arc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73))));
    floor.set_normal_mapping(RtwImage::new("mapping/floor.png"));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.1, 555.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -555.0),
        Arc::new(floor),
    )));

    // let mut back_ground_block =
    //     Mapping::new(Arc::new(Lambertian::new(&Color::new(0.05, 0.05, 0.65))));
    // back_ground_block.set_alpha_mapping(RtwImage::new("mapping/alpha mapping.png"));
    // world.add(Arc::new(Quad::new(
    //     Point3::new(555.0, 0.0, 554.0),
    //     Point3::new(-555.0, 0.0, 0.0),
    //     Point3::new(0.0, 555.0, 0.0),
    //     Arc::new(back_ground_block),
    // )));

    let mut color_ball_mapping =
        Mapping::new(Arc::new(Metal::new(&Color::new(1.0, 1.0, 1.0), 0.5)));
    color_ball_mapping.set_light_mapping(RtwImage::new("mapping/light mapping another.jpg"));
    world.add(Arc::new(Sphere::new(
        Point3::new(370.0, 30.0, 290.0),
        30.0,
        Arc::new(color_ball_mapping),
    )));

    load_model(
        "bloody-woof/bloody-woof.obj",
        "bloody-woof/bloody-woof.mtl",
        &mut world,
        90.0,
        Vec3::new(280.0, 100.0 + 50.0, 400.0),
        300.0,
    );

    load_model(
        "minimalist-weedy/weedy.obj",
        "minimalist-weedy/weedy.mtl",
        &mut world,
        180.0,
        Vec3::new(580.0, 510.0, 530.0),
        3000.0,
    );

    load_model(
        "arknights-warehouse/source/Arknights_Warehouse/Warehouse.obj",
        "arknights-warehouse/source/Arknights_Warehouse/Warehouse.mtl",
        &mut world,
        180.0,
        Vec3::new(300.0, 0.0, 400.0),
        40.0,
    );

    // let red = Arc::new(Lambertian::new(&Color::new(0.65, 0.05, 0.05)));
    // let white = Arc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    // let green = Arc::new(Lambertian::new(&Color::new(0.12, 0.45, 0.15)));
    // world.add(Arc::new(Quad::new(
    //     Point3::new(555.0, 0.0, 0.0),
    //     Point3::new(0.0, 0.0, 555.0),
    //     Point3::new(0.0, 555.0, 0.0),
    //     green.clone(),
    // )));
    // world.add(Arc::new(Quad::new(
    //     Point3::new(0.0, 0.0, 555.0),
    //     Point3::new(0.0, 0.0, -555.0),
    //     Point3::new(0.0, 555.0, 0.0),
    //     red.clone(),
    // )));
    // world.add(Arc::new(Quad::new(
    //     Point3::new(0.0, 555.0, 0.0),
    //     Point3::new(555.0, 0.0, 0.0),
    //     Point3::new(0.0, 0.0, 555.0),
    //     white.clone(),
    // )));
    // world.add(Arc::new(Quad::new(
    //     Point3::new(0.0, 0.0, 555.0),
    //     Point3::new(555.0, 0.0, 0.0),
    //     Point3::new(0.0, 0.0, -555.0),
    //     white.clone(),
    // )));

    let light = Arc::new(DiffuseLight::new_color(&Color::new(35.0, 35.0, 35.0)));
    let light1 = Arc::new(DiffuseLight::new_color(&Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(213.0, 688.799, 127.0),
        Point3::new(130.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 105.0),
        light.clone(),
    )));
    // let feet_light = Arc::new(Quad::new(
    //     Point3::new(555.0, 0.2, 0.0),
    //     Point3::new(-30.0, 0.0, 0.0),
    //     Point3::new(0.0, 0.0, 30.0),
    //     light1.clone(),
    // ));
    // let feet_light = Arc::new(Translate::new(feet_light, Vec3::new(-120.0, 0.0, 220.0)));
    // world.add(feet_light);

    let empty_material: Arc<dyn Material> = Arc::new(Lambertian::new(&Color::new(0.0, 0.0, 0.0)));
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        Point3::new(213.0, 688.799, 127.0),
        Point3::new(130.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 105.0),
        empty_material.clone(),
    )));
    // lights.add(Arc::new(Quad::new(
    //     Point3::new(555.0 - 120.0, 0.2 + 0.0, 0.0 + 220.0),
    //     Point3::new(-30.0, 0.0, 0.0),
    //     Point3::new(0.0, 0.0, 30.0),
    //     empty_material.clone(),
    // )));

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
