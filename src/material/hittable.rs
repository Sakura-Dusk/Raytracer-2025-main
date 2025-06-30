mod aabb;
pub(crate) mod bvh;
pub(crate) mod hittable_list;
pub(crate) mod sphere;

use crate::material::Lambertian;
use crate::rtweekend::vec3;
use std::rc::Rc;

use crate::rtweekend::interval;
use crate::rtweekend::vec3::dot;
use crate::rtweekend::vec3::ray;

#[derive(Clone)]
pub(crate) struct HitRecord {
    pub(crate) p: vec3::Point3,
    pub(crate) normal: vec3::Vec3,
    pub mat: Rc<dyn super::Material>,
    t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: vec3::Point3::default(),
            normal: vec3::Vec3::default(),
            mat: Rc::new(Lambertian::default()),
            t: 0.0,
            front_face: false,
        }
    }
    fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &vec3::Vec3) {
        //must make sure "outward_normal" have UNIT length!
        self.front_face = dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, ray_t: &mut interval::Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> aabb::AABB;
}
