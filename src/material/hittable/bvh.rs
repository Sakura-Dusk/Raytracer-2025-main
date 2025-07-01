use crate::material::hittable::aabb::AABB;
use crate::material::hittable::hittable_list::HittableList;
use crate::material::hittable::{HitRecord, Hittable};
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(objects: HittableList) -> Self {
        let length = objects.objects.len();
        let mut objects = objects;
        Self::build(&mut objects.objects, 0, length)
    }

    fn build(objects: &mut [Arc<dyn Hittable>], start: usize, end: usize) -> Self {
        // 首先计算所有对象的包围盒
        let mut bbox = AABB::EMPTY;
        for object in &objects[start..end] {
            bbox = AABB::new_merge(&bbox, &object.bounding_box());
        }

        // 选择最长轴进行分割
        let axis = bbox.longest_axis();
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => panic!("Invalid axis"),
        };

        let object_span = end - start;

        let (left, right) = match object_span {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => {
                if comparator(&objects[start], &objects[start + 1]).is_lt() {
                    (objects[start].clone(), objects[start + 1].clone())
                } else {
                    (objects[start + 1].clone(), objects[start].clone())
                }
            }
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + object_span / 2;
                (
                    Arc::new(Self::build(objects, start, mid)) as Arc<dyn Hittable>,
                    Arc::new(Self::build(objects, mid, end)) as Arc<dyn Hittable>,
                )
            }
        };

        let bbox = AABB::new_merge(&left.bounding_box(), &right.bounding_box());
        Self { left, right, bbox }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index as i32);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index as i32);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap()
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, *ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            &mut Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
