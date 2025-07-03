use crate::rtweekend::vec3::{Vec3, cross, unit_vector};

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new(n: &Vec3) -> Self {
        let v2 = unit_vector(n);
        let a = if v2.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v1 = unit_vector(&cross(&v2, &a));
        let v0 = cross(&v2, &v1);
        Self { axis: [v0, v1, v2] }
    }

    pub fn u(&self) -> &Vec3 {
        &self.axis[0]
    }

    pub fn v(&self) -> &Vec3 {
        &self.axis[1]
    }

    pub fn w(&self) -> &Vec3 {
        &self.axis[2]
    }

    pub fn transform(&self, v: &Vec3) -> Vec3 {
        *v.index_val(0) * self.axis[0]
            + *v.index_val(1) * self.axis[1]
            + *v.index_val(2) * self.axis[2]
    }
}
