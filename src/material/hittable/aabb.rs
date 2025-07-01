use crate::rtweekend::vec3::Vec3;
use crate::{rtweekend::interval::Interval, rtweekend::vec3::Point3, rtweekend::vec3::ray::Ray};
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub(crate) fn default() -> AABB {
        Self {
            x: Interval::default(),
            y: Interval::default(),
            z: Interval::default(),
        }
    }
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut res = Self { x, y, z };
        res.pad_to_minimums();
        res
    }

    pub fn new_points(a: Point3, b: Point3) -> Self {
        let mut res = Self {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        };
        res.pad_to_minimums();
        res
    }

    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }

    pub fn new_merge(box0: &AABB, box1: &AABB) -> Self {
        Self {
            x: Interval::new_merge(&box0.x, &box1.x),
            y: Interval::new_merge(&box0.y, &box1.y),
            z: Interval::new_merge(&box0.z, &box1.z),
        }
    }

    pub fn axis_interval(&self, axis: i32) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / r.direction.index_val(axis);

            let mut t0 = (ax.min - r.origin.index_val(axis)) * adinv;
            let mut t1 = (ax.max - r.origin.index_val(axis)) * adinv;

            if adinv < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            ray_t.min = ray_t.min.max(t0);
            ray_t.max = ray_t.max.min(t1);

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub(crate) const EMPTY: AABB = AABB {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub(crate) const UNIVERSE: AABB = AABB {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }
}

impl Add<Vec3> for AABB {
    type Output = Self;
    fn add(self, offset: Vec3) -> Self::Output {
        Self::new(self.x + offset.x, self.y + offset.y, self.z + offset.z)
    }
}

impl Add<AABB> for Vec3 {
    type Output = AABB;

    fn add(self, bbox: AABB) -> AABB {
        bbox + self
    }
}
