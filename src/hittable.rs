pub(crate) mod hittable_list;
pub(crate) mod sphere;

use crate::rtweekend::vec3;

use crate::rtweekend::vec3::dot;
use crate::rtweekend::vec3::ray;
use crate::rtweekend::interval;

#[derive(Debug, Copy, Clone)]
pub(crate) struct HitRecord {
    p: vec3::Point3,
    pub(crate) normal: vec3::Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: vec3::Point3::new(0.0, 0.0, 0.0),
            normal: vec3::Vec3::new(0.0, 0.0, 0.0),
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
    fn hit(&self, r: &ray::Ray, ray_t: &interval::Interval, rec: &mut HitRecord) -> bool;
}
