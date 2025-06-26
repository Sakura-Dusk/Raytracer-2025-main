mod camera;
mod hittable;
mod rtweekend;

use rtweekend::vec3::Vec3;

use crate::camera::Camera;
use hittable::hittable_list;
use hittable::sphere;

fn main() {
    //World build
    let mut world: hittable_list::HittableList = hittable_list::HittableList::new();
    world.add(Box::new(sphere::Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Box::new(sphere::Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
