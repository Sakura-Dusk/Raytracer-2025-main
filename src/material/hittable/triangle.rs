use crate::material::Material;
use crate::material::hittable::aabb::AABB;
use crate::material::hittable::{HitRecord, Hittable};
use crate::material::texture::UV;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::random_double;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{Point3, Vec3, cross, dot, unit_vector};
use std::f64::INFINITY;
use std::sync::Arc;

pub struct Triangle {
    q: Point3,
    u: Vec3,
    v: Vec3,
    tq: UV,
    tu: UV,
    tv: UV,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    nq: Vec3,
    nu: Vec3,
    nv: Vec3,
    tangent: Vec3,
    d: f64,
    area: f64,
}

impl Triangle {
    pub(crate) fn new(
        q: Point3,
        u: Vec3,
        v: Vec3,
        tq: UV,
        tu: UV,
        tv: UV,
        nq: Vec3,
        nu: Vec3,
        nv: Vec3,
        mat: Arc<dyn Material>,
    ) -> Self {
        let mut res = Self {
            q,
            u,
            v,
            tq,
            tu: tu.clone(),
            tv: tv.clone(),
            w: Vec3::default(),
            mat,
            bbox: AABB::default(),
            normal: Vec3::default(),
            nq,
            nu,
            nv,
            tangent: Vec3::default(),
            d: 0.0,
            area: 0.0,
        };
        let n = cross(&u, &v);
        res.normal = unit_vector(&n);
        res.d = dot(&res.normal, &q);
        res.w = n / dot(&n, &n);
        res.tangent = (u * tv.v.clone() - v * tu.u.clone())
            / (tu.u.clone() * tv.v.clone() - tu.v.clone() * tv.u.clone());

        res.area = n.length() / 2.0;

        res.set_bounding_box();
        res
    }

    pub(crate) fn new_point(
        x: Point3,
        y: Point3,
        z: Point3,
        tx: UV,
        ty: UV,
        tz: UV,
        nx: Vec3,
        ny: Vec3,
        nz: Vec3,
        mat: Arc<dyn Material>,
    ) -> Self {
        let q = x;
        let u = y - x;
        let v = z - x;
        let tq = tx.clone();
        let tu = ty - tx.clone();
        let tv = tz - tx.clone();
        let nq = nx;
        let nu = ny - nx;
        let nv = nz - nx;
        if dot(&nq, &cross(&u, &v)) > 0.0 {
            Triangle::new(q, u, v, tq, tu, tv, nq, nu, nv, mat)
        } else {
            Triangle::new(q, v, u, tq, tv, tu, nq, nv, nu, mat)
        }
    }

    pub fn set_single_uv(&mut self, uv: UV) {
        self.tq = uv;
        self.tu = UV::default();
        self.tv = UV::default();
    }

    fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = AABB::new_points(self.q, self.q + self.u);
        let bbox_diagonal2 = AABB::new_points(self.q, self.q + self.v);
        self.bbox = AABB::new_merge(&bbox_diagonal1, &bbox_diagonal2);
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

        let uv = self.tq.clone() + self.tu.clone() * alpha + self.tv.clone() * beta;
        let normal =
            unit_vector(&(self.nq.clone() + self.nu.clone() * alpha + self.nv.clone() * beta));

        let tangent = unit_vector(&(self.tangent - normal * dot(&self.tangent, &normal)));
        let bitangent = cross(&normal, &tangent);

        let alpha_ = self.mat.get_alpha_mapping(uv.u, uv.v);
        let x = random_double();
        if x <= alpha {
            return false;
        }

        if self.mat.check_alpha_mapping() == true {
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
