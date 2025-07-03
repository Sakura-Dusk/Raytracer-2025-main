mod camera;
mod material;
mod pdf;
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
use std::sync::Arc;

fn main() {
    cornell_box();
}
fn cornell_box() {
    let mut world: HittableList = HittableList::new();

    let red = Arc::new(Lambertian::new(&Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(&Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(&Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(&Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 555.0),
        Point3::new(0.0, 555.0, 0.0),
        green.clone(),
    )));
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

    let mut box2: Arc<dyn Hittable> = make_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let empty_material: Arc<dyn Material> = Arc::new(Lambertian::new(&Color::new(0.0, 0.0, 0.0)));
    let lights = Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        empty_material,
    ));

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world, lights);
}
