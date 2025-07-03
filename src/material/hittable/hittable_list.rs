use crate::material::hittable::aabb::AABB;
use crate::material::hittable::{HitRecord, Hittable};
use crate::rtweekend::interval::Interval;
use crate::rtweekend::random_int_range;
use crate::rtweekend::vec3::Vec3;
use crate::rtweekend::vec3::ray::Ray;
use std::sync::Arc;

pub(crate) struct HittableList {
    pub(crate) objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        let bbox = object.bounding_box();
        self.objects.push(object);
        self.bbox = AABB::new_merge(&self.bbox, &bbox);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t: &mut Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t.max;

        for object in &self.objects {
            if object.hit(
                &ray,
                &mut Interval::new(t.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in self.objects.clone() {
            sum += weight * object.pdf_value(origin, direction);
        }

        sum
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        self.objects[random_int_range(0, int_size - 1) as usize].random(origin)
    }
}
