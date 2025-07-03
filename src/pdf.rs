use crate::material::hittable::Hittable;
use crate::material::onb::Onb;
use crate::rtweekend::vec3::{
    Point3, Vec3, dot, random_cosine_direction, random_unit_vector, unit_vector,
};
use crate::rtweekend::{PI, random_double};
use std::sync::Arc;

pub trait Pdf: Send + Sync {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

#[derive(Default)]
pub struct SpherePdf {}

impl Pdf for SpherePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: Onb::new(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = dot(&unit_vector(direction), self.uvw.w());
        cosine_theta / PI.max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(&random_cosine_direction())
    }
}

pub struct HittablePdf {
    objects: Arc<dyn Hittable>,
    origin: Point3,
}

impl HittablePdf {
    pub fn new(objects: Arc<dyn Hittable>, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

pub struct MixturePdf {
    p: [Arc<dyn Pdf>; 2],
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
