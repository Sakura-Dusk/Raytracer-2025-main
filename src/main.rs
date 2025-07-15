mod camera;
mod material;
mod pdf;
mod rtweekend;

use crate::camera::Camera;
use crate::material::hittable::bvh::BvhNode;
use crate::material::hittable::hittable_list::HittableList;
use crate::material::hittable::quad::{Quad, make_box};
use crate::material::hittable::sphere::Sphere;
use crate::material::hittable::triangle::Triangle;
use crate::material::hittable::{Hittable, RotateY, Translate};
use crate::material::texture::model::load_model;
use crate::material::texture::rtw_stb_image::RtwImage;
use crate::material::texture::{ImageTexture, SolidColor};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Mapping, Material, Metal};
use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::{Point3, cross, random_unit_vector, unit_vector};
use crate::rtweekend::{PI, random_double, random_double_range};
use rtweekend::vec3::Vec3;
use std::sync::Arc;
use std::sync::TryLockError::Poisoned;
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
        Point3::new(-300.0, 0.1, 755.0),
        Point3::new(1355.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -755.0),
        Arc::new(floor),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(-300.0, 0.0, 755.0),
        Point3::new(1355.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -755.0),
        Arc::new(Lambertian::new(&Color::new(0.1, 0.1, 0.1))),
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
        "arknights-warehouse/source/Arknights_Warehouse/Warehouse.obj",
        "arknights-warehouse/source/Arknights_Warehouse/Warehouse.mtl",
        &mut world,
        180.0,
        Vec3::new(300.0, 0.0, 400.0),
        40.0,
    );

    let light = Arc::new(DiffuseLight::new_color(&Color::new(35.0, 35.0, 35.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(213.0, 688.799, 127.0),
        Point3::new(130.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 105.0),
        light.clone(),
    )));

    let empty_material: Arc<dyn Material> = Arc::new(Lambertian::new(&Color::new(0.0, 0.0, 0.0)));
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        Point3::new(213.0, 688.799, 127.0),
        Point3::new(130.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 105.0),
        empty_material.clone(),
    )));

    let metal = Arc::new(Metal::new(&Color::new(1.0, 1.0, 1.0), 0.0));
    world.add(Arc::new(Quad::new(
        Point3::new(-70.0, 0.0, -500.0),
        Point3::new(0.0, 0.0, 1500.0),
        Point3::new(0.0, 1000.0, 0.0),
        metal.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(700.0, 0.0, -500.0),
        Point3::new(0.0, 1000.0, 0.0),
        Point3::new(0.0, 0.0, 1500.0),
        metal.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(700.0, 900.0, 0.0),
        Point3::new(0.0, 0.0, 1000.0),
        Point3::new(-1000.0, 0.0, 0.0),
        metal.clone(),
    )));

    let mut backwall_mapping = Mapping::default();
    backwall_mapping.set_light_mapping(RtwImage::new("backwall.png"));
    let backwall_object = Arc::new(Quad::new(
        Point3::new(-70.0, 0.0, 1000.0),
        Point3::new(0.0, 1000.0, 0.0),
        Point3::new(1000.0, 0.0, 0.0),
        Arc::new(backwall_mapping),
    ));
    world.add(backwall_object);

    let mut jupiter = Mapping::default();
    jupiter.set_light_mapping(RtwImage::new("Jupiter.png"));
    let jupiter_object = Arc::new(Sphere::new(
        Point3::new(70.0, 400.0, 150.0),
        50.0,
        Arc::new(jupiter),
    ));
    world.add(jupiter_object);

    let mut lava_floor = Mapping::new(Arc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73))));
    lava_floor.set_light_mapping(RtwImage::new("lava.jpg"));
    world.add(make_box(
        &Point3::new(-10.0, 0.2, 20.0),
        &Point3::new(-10.0 + 80.0, 0.2 + 80.0, 20.0 + 80.0),
        Arc::new(lava_floor),
    ));
    // world.add(Arc::new(Quad::new(
    //     Point3::new(-10.0, 0.2, 20.0),
    //     Point3::new(0.0, 0.0, 80.0),
    //     Point3::new(80.0, 0.0, 0.0),
    //     Arc::new(lava_floor.clone()),
    // )));
    let mut glass_world = HittableList::new();
    for i in 1..50 {
        let glass = Arc::new(Dielectric::new(random_double_range(0.0, 2.0)));
        let x = Point3::new(
            000.0 + random_double_range(0.0, 3.0 * i as f64),
            80.0 + 0.42 * i as f64 + random_double_range(0.0, 7.0 * i as f64),
            20.0 + random_double_range(0.0, 5.0 * i as f64),
        );
        glass_world.add(Arc::new(Sphere::new(
            x,
            random_double_range(1.0, 1.0 + i as f64 * 0.42),
            glass.clone(),
        )));
        // let y = random_double_range(1.0, 10.0) * random_unit_vector();
        // let z = random_double_range(1.0, 10.0) * random_unit_vector();
        // let n = unit_vector(&cross(&y, &z));
        // world.add(Arc::new(Quad::new(x, y, z, metal.clone())));
        // world.add(Arc::new(Quad::new(x - n * 0.01, z, y, metal.clone())));
    }
    world.add(Arc::new(BvhNode::new(glass_world)));

    let mut circle_world = HittableList::new();
    let circle_number = 100;
    for i in 0..circle_number {
        let y = random_double_range(-5.0, 5.0);
        let mp = Point3::new(300.0, 700.0 + y, 300.0);
        let len = random_double_range(350.0, 400.0);
        // let dx = Point3::new(1.0, 0.0, 0.0);
        // let dy = Point3::new(0.0, -0.8, -0.6);
        let dx = Point3::new(1.0, 0.0, 0.0);
        let dy = Point3::new(0.0, 0.0, 1.0);
        let x = mp
            + len
                * (dx * (2.0 * PI * i as f64 / circle_number as f64).sin()
                    + dy * (2.0 * PI * i as f64 / circle_number as f64).cos());
        let sz = random_double_range(5.0, 10.0);

        let rd = random_double();
        let cl = Color::new(
            random_double_range(3.0, 5.0),
            random_double_range(3.0, 5.0),
            random_double_range(3.0, 5.0),
        );
        // if rd < 0.3 {cl = Color::new(random_double_range(3.0, 5.0), 0.0, 0.0);}
        // else if rd < 0.6 {cl = Color::new(0.0, 0.0, random_double_range(3.0, 5.0));}
        // else if rd < 0.9 {cl = Color::new(0.0, random_double_range(3.0, 5.0), 0.0);}
        // else {cl = Color::new(random_double_range(3.0, 5.0), random_double_range(3.0, 5.0), random_double_range(3.0, 5.0));}

        if rd < 0.3 {
            circle_world.add(Arc::new(Sphere::new(
                x,
                sz,
                Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(&cl)))),
            )));
            // lights.add(Arc::new(Sphere::new(
            //     x,
            //     sz,
            //     empty_material.clone(),
            // )));
        } else if rd < 0.7 {
            circle_world.add(Arc::new(Sphere::new(
                x,
                sz,
                Arc::new(Metal::new(
                    &Color::new(random_double(), random_double(), random_double()),
                    random_double_range(0.0, 0.2),
                )),
            )));
        } else {
            circle_world.add(Arc::new(Sphere::new(
                x,
                sz,
                Arc::new(Dielectric::new(random_double_range(0.0, 2.0))),
            )));
        }
    }
    world.add(Arc::new(BvhNode::new(circle_world)));

    // world.add(Arc::new(Quad::new(
    //     Point3::new(-1000.0, -1000.0, -900.0),
    //     Point3::new(2000.0, 0.0, 0.0),
    //     Point3::new(0.0, 2000.0, 0.0),
    //     Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(&Color::new(1.0, 1.0, 1.0))))),
    // )));

    world.add(Arc::new(Sphere::new(
        Point3::new(490.0, 155.0, -50.0),
        100.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    let mut inside_world = HittableList::new();
    let inside_number = 78;
    for i in 0..inside_number {
        let phi = random_double_range(0.0, PI);
        let theta = random_double_range(0.0, 2.0 * PI);

        let r = random_double_range(50.0, 90.0);
        let rin = random_double_range(20.0, r);

        let x = Point3::new(
            r * phi.sin(),
            r * phi.cos() * theta.sin(),
            r * phi.cos() * theta.cos(),
        ) + Point3::new(490.0, 155.0, -50.0);
        let y = Point3::new(
            rin * phi.sin(),
            rin * phi.cos() * theta.sin(),
            rin * phi.cos() * theta.cos(),
        ) + Point3::new(490.0, 155.0, -50.0);

        let sz = random_double_range(4.0, 9.0);

        let rnd = random_double();

        if rnd < 0.5 {
            inside_world.add(Arc::new(Sphere::new_move(
                x,
                y,
                sz,
                Arc::new(Lambertian::new(&Color::new(
                    random_double(),
                    random_double(),
                    random_double(),
                ))),
            )));
        } else if rnd < 0.9 {
            inside_world.add(Arc::new(Sphere::new_move(
                x,
                y,
                sz,
                Arc::new(Metal::new(
                    &Color::new(random_double(), random_double(), random_double()),
                    random_double(),
                )),
            )));
        } else {
            inside_world.add(Arc::new(Sphere::new_move(
                x,
                y,
                sz,
                Arc::new(Dielectric::new(random_double_range(0.0, 2.0))),
            )));
        }
    }
    world.add(Arc::new(BvhNode::new(inside_world)));

    world.add(Arc::new(Sphere::new(
        Point3::new(600.0, 155.0, -250.0),
        50.0,
        Arc::new(Metal::new(&Color::new(0.8, 0.8, 0.8), 0.0)),
    )));

    let mut background_back = Mapping::new(Arc::new(Lambertian::new(&Color::new(0.1, 0.1, 0.1))));
    background_back.set_light_mapping(RtwImage::new("background_back.png"));
    world.add(Arc::new(Quad::new(
        Point3::new(-1500.0, -1500.0, -1000.0),
        Point3::new(3000.0, 0.0, 0.0),
        Point3::new(0.0, 3000.0, 0.0),
        Arc::new(background_back),
        // Arc::new(Lambertian::new_tex(Arc::new(ImageTexture::new("background_back.png"))))
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1600;
    cam.samples_per_pixel = 2000;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world, Arc::new(lights));
}
