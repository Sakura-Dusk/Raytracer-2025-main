mod aabb;
pub(crate) mod bvh;
pub(crate) mod constant_medium;
pub(crate) mod hittable_list;
pub(crate) mod quad;
pub(crate) mod sphere;

use crate::material::Lambertian;
use crate::material::hittable::aabb::AABB;
use crate::rtweekend::interval;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::ray;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{Point3, Vec3, dot};
use crate::rtweekend::{degrees_to_radians, vec3};
use std::f64::{INFINITY, NEG_INFINITY};
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct HitRecord {
    pub(crate) p: vec3::Point3,
    pub(crate) normal: Vec3,
    pub mat: Rc<dyn super::Material>,
    t: f64,
    pub(crate) u: f64,
    pub(crate) v: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: vec3::Point3::default(),
            normal: Vec3::default(),
            mat: Rc::new(Lambertian::default()),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
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
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABB;
}

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Translate {
        let bbox = object.bounding_box() + offset;
        Translate {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new_move(r.origin - self.offset, r.direction, r.time);

        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;

        true
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    min.x = min.x.min(tester.x);
                    max.x = max.x.max(tester.x);
                    min.y = min.y.min(tester.y);
                    max.y = max.y.max(tester.y);
                    min.z = min.z.min(tester.z);
                    max.z = max.z.max(tester.z);
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: AABB::new_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            self.cos_theta * r.origin.x - self.sin_theta * r.origin.z,
            r.origin.y,
            self.sin_theta * r.origin.x + self.cos_theta * r.origin.z,
        );

        let direction = Vec3::new(
            self.cos_theta * r.direction.x - self.sin_theta * r.direction.z,
            r.direction.y,
            self.sin_theta * r.direction.x + self.cos_theta * r.direction.z,
        );

        let rotated_r = Ray::new_move(origin, direction, r.time);

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
            rec.p.y,
            -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
        );

        rec.normal = Vec3::new(
            self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
            rec.normal.y,
            -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
        );

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
