mod camera;
mod material;
mod rtweekend;

use rtweekend::vec3::Vec3;
use std::rc::Rc;

use crate::camera::Camera;
use crate::material::Lambertian;
use crate::material::hittable::sphere::Sphere;
use crate::rtweekend::color::Color;
use material::hittable::hittable_list;

fn main() {
    //World build
    let mut world: hittable_list::HittableList = hittable_list::HittableList::new();

    let R = (rtweekend::PI / 4.0).cos();

    let material_left = Rc::new(Lambertian::new(&Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(&Color::new(1.0, 0.0, 0.0)));

    world.add(Box::new(Sphere {
        center: Vec3::new(-R, 0.0, -1.0),
        radius: R,
        mat: material_left,
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(R, 0.0, -1.0),
        radius: R,
        mat: material_right,
    }));

    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
