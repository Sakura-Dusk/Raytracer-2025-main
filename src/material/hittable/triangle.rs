use crate::material::Material;
use crate::material::hittable::aabb::Aabb;
use crate::material::hittable::{HitRecord, Hittable};
use crate::material::texture::UV;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::random_double;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{Point3, Vec3, cross, dot, unit_vector};
use std::sync::Arc;

pub struct Triangle {
    q: Point3,
    u: Vec3,
    v: Vec3,
    tquv: Tquv,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    nquv: Nquv,
    tangent: Vec3,
    d: f64,
    area: f64,
}

#[derive(Clone)]
pub struct Tquv {
    tq: UV,
    tu: UV,
    tv: UV,
}

impl Tquv {
    pub(crate) fn new(tq: UV, tu: UV, tv: UV) -> Tquv {
        Tquv { tq, tu, tv }
    }
}

#[derive(Clone)]
pub struct Nquv {
    nq: Vec3,
    nu: Vec3,
    nv: Vec3,
}

impl Nquv {
    pub(crate) fn new(nq: Vec3, nu: Vec3, nv: Vec3) -> Nquv {
        Nquv { nq, nu, nv }
    }
}

impl Triangle {
    pub(crate) fn new(
        q: Point3,
        u: Vec3,
        v: Vec3,
        tquv: Tquv,
        nquv: Nquv,
        mat: Arc<dyn Material>,
    ) -> Self {
        let mut res = Self {
            q,
            u,
            v,
            tquv: tquv.clone(),
            nquv: nquv.clone(),
            w: Vec3::default(),
            mat,
            bbox: Aabb::default(),
            normal: Vec3::default(),
            tangent: Vec3::default(),
            d: 0.0,
            area: 0.0,
        };
        let n = cross(&u, &v);
        res.normal = unit_vector(&n);
        res.d = dot(&res.normal, &q);
        res.w = n / dot(&n, &n);
        res.tangent =
            (u * tquv.tv.v - v * tquv.tu.u) / (tquv.tu.u * tquv.tv.v - tquv.tu.v * tquv.tv.u);

        res.area = n.length() / 2.0;

        res.set_bounding_box();
        res
    }

    pub(crate) fn new_point(
        x: Point3,
        y: Point3,
        z: Point3,
        tquv: Tquv,
        nquv: Nquv,
        mat: Arc<dyn Material>,
    ) -> Self {
        let q = x;
        let u = y - x;
        let v = z - x;
        let tq = tquv.tq.clone();
        let tu = tquv.tu - tquv.tq.clone();
        let tv = tquv.tv - tquv.tq.clone();
        let nq = nquv.nq;
        let nu = nquv.nu - nquv.nq;
        let nv = nquv.nv - nquv.nq;
        if dot(&nq, &cross(&u, &v)) > 0.0 {
            Triangle::new(q, u, v, Tquv::new(tq, tu, tv), Nquv::new(nq, nu, nv), mat)
        } else {
            Triangle::new(q, v, u, Tquv::new(tq, tv, tu), Nquv::new(nq, nv, nu), mat)
        }
    }

    // pub fn set_single_uv(&mut self, uv: UV) {
    //     self.tquv.tq = uv;
    //     self.tquv.tu = UV::default();
    //     self.tquv.tv = UV::default();
    // }

    fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = Aabb::new_points(self.q, self.q + self.u);
        let bbox_diagonal2 = Aabb::new_points(self.q, self.q + self.v);
        self.bbox = Aabb::new_merge(&bbox_diagonal1, &bbox_diagonal2);
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a)
            || !unit_interval.contains(b)
            || !unit_interval.contains(a + b)
        {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Triangle {
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

        if !Triangle::is_interior(alpha, beta, rec) {
            return false;
        }

        let uv = self.tquv.tq.clone() + self.tquv.tu.clone() * alpha + self.tquv.tv.clone() * beta;
        let normal = unit_vector(&(self.nquv.nq + self.nquv.nu * alpha + self.nquv.nv * beta));

        let tangent = unit_vector(&(self.tangent - normal * dot(&self.tangent, &normal)));
        let bitangent = cross(&normal, &tangent);

        let _ = self.mat.get_alpha_mapping(uv.u, uv.v);
        let x = random_double();
        if x <= alpha {
            return false;
        }

        if self.mat.check_alpha_mapping() {
            let stop_p = self.mat.get_alpha_mapping(alpha, beta);
            if random_double() < stop_p {
                return false;
            }
        }

        //Ray hits the 2D shape
        rec.u = uv.u;
        rec.v = uv.v;
        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.tangent = tangent;
        rec.bitangent = bitangent;
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::new();
        if !self.hit(
            &Ray::new(*origin, *direction),
            &mut Interval::new(0.001, f64::INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = dot(direction, &self.normal).abs() / direction.length();

        distance_squared / (cosine * self.area)
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let mut vx = random_double();
        let mut vy = random_double();
        if vx + vy > 1.0 {
            vx = 1.0 - vx;
            vy = 1.0 - vy;
        }
        let p = self.q + (vx * self.u) + (vy * self.v);
        p - *origin
    }
}

pub struct TriangleSingle {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    area: f64,
}

impl TriangleSingle {
    // pub(crate) fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
    //     let mut res = Self {
    //         q,
    //         u,
    //         v,
    //         w: Vec3::default(),
    //         mat,
    //         bbox: Aabb::default(),
    //         normal: Vec3::default(),
    //         d: 0.0,
    //         area: 0.0,
    //     };
    //     let n = cross(&u, &v);
    //     res.normal = unit_vector(&n);
    //     res.d = dot(&res.normal, &q);
    //     res.w = n / dot(&n, &n);
    //
    //     res.area = n.length() / 2.0;
    //
    //     res.set_bounding_box();
    //     res
    // }
    //
    // fn set_bounding_box(&mut self) {
    //     let bbox_line1 = Aabb::new_points(self.q, self.q + self.u);
    //     let bbox_line2 = Aabb::new_points(self.q, self.q + self.v);
    //     self.bbox = Aabb::new_merge(&bbox_line1, &bbox_line2);
    // }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a)
            || !unit_interval.contains(b)
            || !unit_interval.contains(a + b)
        {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for TriangleSingle {
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

        if !TriangleSingle::is_interior(alpha, beta, rec) {
            return false;
        }

        if self.mat.check_alpha_mapping() {
            let stop_p = self.mat.get_alpha_mapping(alpha, beta);
            if random_double() < stop_p {
                return false;
            }
        }

        //Ray hits the 2D shape
        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal, &self.mat, alpha, beta);

        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::new();
        if !self.hit(
            &Ray::new(*origin, *direction),
            &mut Interval::new(0.001, f64::INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = dot(direction, &self.normal).abs() / direction.length();

        distance_squared / (cosine * self.area)
    }
}

// pub fn make_box(a: &Point3, b: &Point3, c: &Point3, h: f64, mat: Arc<dyn Material>) -> Arc<HittableList> {
//     let mut sides = HittableList::new();
//     let w = unit_vector(&cross(b, c));
//
//     sides.add(Arc::new(TriangleSingle::new(*a, *b, *c, mat.clone())));
//
//
//     Arc::new(sides)
// }
