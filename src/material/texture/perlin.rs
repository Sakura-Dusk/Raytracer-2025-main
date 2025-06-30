use crate::rtweekend::vec3::Vec3;
use crate::rtweekend::{random_double, random_int_range};
use std::ops::Index;

const POINT_COUNT: usize = 256;
pub(crate) struct Perlin {
    randfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Default for Perlin {
    fn default() -> Self {
        let mut randfloat = Vec::with_capacity(POINT_COUNT);
        for i in 0..POINT_COUNT {
            randfloat.push(random_double());
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Perlin {
    pub fn noise(&self, p: &Vec3) -> f64 {
        let i = ((4.0 * p.x) as i32) & 255;
        let j = ((4.0 * p.y) as i32) & 255;
        let k = ((4.0 * p.z) as i32) & 255;

        self.randfloat[(self.perm_x.index(i as usize)
            ^ self.perm_y.index(j as usize)
            ^ self.perm_z.index(k as usize)) as usize]
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(POINT_COUNT);
        for i in 0..256 {
            p.push(i as i32);
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut Vec<i32>, n: usize) {
        for i in (0..n).rev() {
            let target = random_int_range(0, i as i32) as usize;
            p.swap(i, target);
        }
    }
}
