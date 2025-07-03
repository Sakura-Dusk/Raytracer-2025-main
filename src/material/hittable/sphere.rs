use crate::material::Material;
use crate::material::hittable::aabb::AABB;
use crate::material::hittable::{HitRecord, Hittable};
use crate::material::onb::Onb;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::ray::Ray;
use crate::rtweekend::vec3::{Point3, Vec3};
use crate::rtweekend::{PI, random_double, vec3};
use std::f64::INFINITY;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct Sphere {
    pub(crate) center: Ray,
    pub(crate) radius: f64,
    pub(crate) mat: Arc<dyn Material>,
    pub(crate) bbox: AABB,
}

impl Sphere {
    pub(crate) fn new(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius: radius.max(0.0),
            mat,
            bbox: AABB::new_points(static_center - rvec, static_center + rvec),
        }
    }

    pub(crate) fn new_move(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Arc<dyn Material>,
    ) -> Self {
        let center = Ray::new(center1, center2 - center1);
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center,
            radius: radius.max(0.0),
            mat,
            bbox: AABB::new_merge(
                &AABB::new_points(center.at(0.0) - rvec, center.at(0.0) + rvec),
                &AABB::new_points(center.at(1.0) - rvec, center.at(1.0) + rvec),
            ),
        }
    }

    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }

    fn random_to_sphere(&self, radius: f64, distance_squared: f64) -> Vec3 {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        Vec3::new(x, y, z)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time);
        let oc = current_center - r.origin;
        let a = r.direction.length_squared();
        let h = vec3::dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a; //find small root first
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        (rec.u, rec.v) = Self::get_sphere_uv(&outward_normal);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        //Only works for stationary spheres

        let mut rec = HitRecord::new();
        if !self.hit(
            &Ray::new(*origin, *direction),
            &mut Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let dist_squared = (self.center.at(0.0) - origin.clone()).length_squared();
        let cos_theta_max = (1.0 - self.radius * self.radius / dist_squared).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let direction = self.center.at(0.0) - origin.clone();
        let distance_squared = direction.length_squared();
        let uvw = Onb::new(&direction);
        uvw.transform(&self.random_to_sphere(self.radius, distance_squared))
    }
}
