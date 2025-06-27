mod camera;
mod material;
mod rtweekend;

use rtweekend::vec3::Vec3;
use std::rc::Rc;

use crate::camera::Camera;
use crate::material::hittable::sphere;
use crate::rtweekend::color;
use crate::rtweekend::vec3::Point3;
use material::hittable::hittable_list;

fn main() {
    //World build
    let mut world: hittable_list::HittableList = hittable_list::HittableList::new();

    let material_ground = Rc::new(material::Lambertian::new(&color::Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Lambertian::new(&color::Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(material::Dielectric::new(1.50));
    let material_bubble = Rc::new(material::Dielectric::new(1.00 / 1.50));
    let material_right = Rc::new(material::Metal::new(&color::Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(sphere::Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: material_ground,
    }));
    world.add(Box::new(sphere::Sphere {
        center: Vec3::new(0.0, 0.0, -1.2),
        radius: 0.5,
        mat: material_center,
    }));
    world.add(Box::new(sphere::Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat: material_left,
    }));
    world.add(Box::new(sphere::Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.4,
        mat: material_bubble,
    }));
    world.add(Box::new(sphere::Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        mat: material_right,
    }));

    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}
