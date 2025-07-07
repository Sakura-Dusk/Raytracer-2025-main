use crate::material::hittable::Hittable;
use crate::material::hittable::bvh::BvhNode;
use crate::material::hittable::hittable_list::HittableList;
use crate::material::hittable::triangle::Triangle;
use crate::material::{Lambertian, Material};
use crate::rtweekend::vec3::Vec3;
use std::sync::Arc;

pub fn get_models(models_name: &str, scale: f64) -> Arc<dyn Hittable> {
    let mut res = HittableList::new();
    let obj = tobj::load_obj(
        format!("images/{}", models_name),
        &tobj::LoadOptions {
            single_index: false,
            triangulate: false,
            ignore_points: true,
            ignore_lines: true,
        },
    );
    let (models, materials) = obj.expect("Failed to load obj");
    let materials = materials.expect("Failed to load mtl");
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        let mut mat: Arc<dyn Material> = Arc::new(Lambertian::new(&Vec3::new(0.0, 0.0, 0.0)));
        let id = mesh.material_id;
        if let Some(diffuse) = materials.clone()[id.unwrap()].diffuse {
            mat = Arc::new(Lambertian::new(&Vec3::new(
                diffuse[0] as f64,
                diffuse[1] as f64,
                diffuse[2] as f64,
            )));
        }
        if !mesh.face_arities.is_empty() {
            let mut next_face = 0;
            for f in 0..mesh.face_arities.len() {
                let end = next_face + mesh.face_arities[f] as usize;
                let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
                next_face = end;
                let mut point: [Vec3; 3] = [Vec3::new(0.0, 0.0, 0.0); 3];
                let mut t = 0;
                let p0 = *face_indices[0] as usize;
                point[0] = Vec3::new(
                    mesh.positions[3 * p0] as f64 * scale,
                    mesh.positions[3 * p0 + 1] as f64 * scale,
                    mesh.positions[3 * p0 + 2] as f64 * scale,
                );
                for v in face_indices {
                    t += 1;
                    point[1] = point[2];
                    point[2] = Vec3::new(
                        mesh.positions[3 * (*v as usize)] as f64 * scale,
                        mesh.positions[3 * (*v as usize) + 1] as f64 * scale,
                        mesh.positions[3 * (*v as usize) + 2] as f64 * scale,
                    );
                    if t >= 3 {
                        res.add(Arc::new(Triangle::new_point(
                            point[0],
                            point[1],
                            point[2],
                            mat.clone(),
                        )));
                    }
                }
            }
        } else {
            let mut point: [Vec3; 3] = [Vec3::default(); 3];
            let mut t = 0;
            for v in &mesh.indices {
                point[t] = Vec3::new(
                    mesh.positions[3 * (*v as usize)] as f64 * scale,
                    mesh.positions[3 * (*v as usize) + 1] as f64 * scale,
                    mesh.positions[3 * (*v as usize) + 2] as f64 * scale,
                );
                t += 1;
                if t == 3 {
                    res.add(Arc::new(Triangle::new_point(
                        point[0],
                        point[1],
                        point[2],
                        mat.clone(),
                    )));
                    t = 0;
                }
            }
        }
    }

    let mut bvh_list = Arc::new(BvhNode::new(res));
    bvh_list
}
