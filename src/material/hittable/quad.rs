use crate::material::Material;
use crate::material::hittable::aabb::AABB;
use crate::material::hittable::hittable_list::HittableList;
use crate::material::hittable::{HitRecord, Hittable};
use crate::rtweekend::interval::Interval;
use crate::rtweekend::random_double;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{Point3, Vec3, cross, dot, unit_vector};
use std::f64::INFINITY;
use std::sync::Arc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
    area: f64,
}

impl Quad {
    pub(crate) fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let mut res = Self {
            q,
            u,
            v,
            w: Vec3::default(),
            mat,
            bbox: AABB::default(),
            normal: Vec3::default(),
            d: 0.0,
            area: 0.0,
        };
        let n = cross(&u, &v);
        res.normal = unit_vector(&n);
        res.d = dot(&res.normal, &q);
        res.w = n / dot(&n, &n);

        res.area = n.length();

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
        rec.set_face_normal(r, &self.normal, &self.mat, alpha, beta);

        true
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::new();
        if !self.hit(
            &Ray::new(*origin, *direction),
            &mut Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = dot(direction, &self.normal).abs() / direction.length();

        distance_squared / (cosine * self.area)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let p = self.q + (random_double() * self.u) + (random_double() * self.v);
        p - *origin
    }
}

pub fn make_box(a: &Point3, b: &Point3, mat: Arc<dyn Material>) -> Arc<HittableList> {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    ))); //front
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    ))); //right
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    ))); //back
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    ))); //left
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        -dz,
        mat.clone(),
    ))); //top
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        mat.clone(),
    ))); //bottom

    Arc::new(sides)
}
