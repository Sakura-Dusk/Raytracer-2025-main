use crate::material::Material;
use crate::material::hittable::aabb::AABB;
use crate::material::hittable::{HitRecord, Hittable};
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{Point3, Vec3, cross, dot, unit_vector};
use std::rc::Rc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Rc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub(crate) fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let mut res = Self {
            q,
            u,
            v,
            w: Vec3::default(),
            mat,
            bbox: AABB::default(),
            normal: Vec3::default(),
            d: 0.0,
        };
        let n = cross(&u, &v);
        res.normal = unit_vector(&n);
        res.d = dot(&res.normal, &q);
        res.w = n / dot(&n, &n);

        res.set_bounding_box();
        res
    }

    fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = AABB::new_points(self.q, self.q + self.u + self.v);
        let bbox_diagonal2 = AABB::new_points(self.q + self.u, self.q + self.v);
        self.bbox = AABB::new_merge(&bbox_diagonal1, &bbox_diagonal2);
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let denom = dot(&self.normal, &r.direction);

        //No hit
        if denom.abs() < 1e-8 {
            return false;
        }

        //hit point parameter is outside the ray interval
        let t = (self.d - dot(&self.normal, &r.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));

        if !Quad::is_interior(alpha, beta, rec) {
            return false;
        }

        //Ray hits the 2D shape
        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);

        true
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
