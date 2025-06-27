use crate::rtweekend::random_double;
use crate::rtweekend::random_double_range;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

pub mod ray;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub(crate) fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }
}

pub type Point3 = Vec3;

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self::Output {
        Self::new(self.x * t, self.y * t, self.z * t)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, t: Vec3) -> Vec3 {
        t * self
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, t: Self) -> Self::Output {
        Vec3::new(self.x * t.x, self.y * t.y, self.z * t.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self::Output {
        Self::new(self.x / t, self.y / t, self.z / t)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.x /= t;
    }
}

pub(crate) fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub(crate) fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    let len = v.length();
    Vec3 {
        x: v.x / len,
        y: v.y / len,
        z: v.z / len,
    }
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, &normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2f64 * dot(v, n) * *n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-*uv, n).min(1.0);
    let r_out_prep = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_prep.length_squared()).abs().sqrt() * *n;
    r_out_prep + r_out_parallel
}
