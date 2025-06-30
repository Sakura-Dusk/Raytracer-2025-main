use crate::material::hittable::aabb::AABB;
use crate::material::hittable::{HitRecord, Hittable};
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::ray::Ray;
use std::rc::Rc;

pub(crate) struct HittableList {
    pub(crate) objects: Vec<Rc<dyn Hittable>>,
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

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
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
}
