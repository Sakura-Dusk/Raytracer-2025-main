use crate::rtweekend::vec3::{Vec3, dot, unit_vector};
use crate::rtweekend::{random_double, random_int_range};
use std::ops::Index;

const POINT_COUNT: usize = 256;
pub(crate) struct Perlin {
    randvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Default for Perlin {
    fn default() -> Self {
        let mut randvec = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            randvec.push(unit_vector(&Vec3::random_range(-1.0, 1.0)));
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Perlin {
    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = *self.randvec.index(
                        (self.perm_x[((i + di as i32) & 255) as usize]
                            ^ self.perm_y[((j + dj as i32) & 255) as usize]
                            ^ self.perm_z[((k + dk as i32) & 255) as usize])
                            as usize,
                    );
                }
            }
        }

        self.perlin_interp(&c, u, v, w)
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(POINT_COUNT);
        for i in 0..256 {
            p.push(i);
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

    fn perlin_interp(&self, c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * dot(&c[i][j][k], &weight_v);
                }
            }
        }
        accum
    }
}
