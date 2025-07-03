use crate::material::onb::Onb;
use crate::rtweekend::PI;
use crate::rtweekend::vec3::{Vec3, dot, random_cosine_direction, random_unit_vector, unit_vector};

pub trait Pdf: Send + Sync {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

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
