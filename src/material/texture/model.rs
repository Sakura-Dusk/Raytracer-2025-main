use crate::material::Lambertian;
use crate::material::hittable::bvh::BvhNode;
use crate::material::hittable::hittable_list::HittableList;
use crate::material::hittable::triangle::Triangle;
use crate::material::hittable::{RotateY, Translate};
use crate::material::texture::UV;
use crate::material::texture::mtl::{create_texture, process_mtl_file};
use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::{Point3, Vec3};
use std::collections::HashMap;
use std::sync::Arc;
use tobj::LoadOptions;

pub fn load_obj(obj_path: &str, mtl_path: &str, scale: f64) -> Vec<Triangle> {
    let (models, materials) = tobj::load_obj(
        &format!("images/{}", obj_path),
        &LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )
    .expect("Failed to load obj");
    let materials = materials.unwrap();

    let mut res_map = HashMap::new();

    if let Ok(parsed) =
        std::panic::catch_unwind(|| process_mtl_file(&format!("images/{}", mtl_path)))
    {
        for (_, info) in parsed {
            let tex = create_texture(&info);
            let mat = Arc::new(Lambertian::new_tex(tex));
            res_map.insert(info.name.clone(), mat);
        }
    }

    let mut triangles = Vec::new();

    for model in &models {
        let mesh = &model.mesh;
        let positions = &mesh.positions;
        let normals = &mesh.normals;
        let texcoords = &mesh.texcoords;
        let indices = &mesh.indices;

        let material = if let Some(mat_id) = mesh.material_id {
            let obj_material = materials.get(mat_id);
            if let Some(mat) = obj_material {
                if let Some(name) = res_map.get(&mat.name) {
                    name.clone()
                } else {
                    Arc::new(Lambertian::new(&Color::new(0.8, 0.0, 0.0)))
                }
            } else {
                Arc::new(Lambertian::new(&Color::new(0.0, 0.8, 0.0)))
            }
        } else {
            Arc::new(Lambertian::new(&Color::new(0.0, 0.0, 0.8)))
        };

        for i in (0..indices.len()).step_by(3) {
            let v0 = Point3::new(
                positions[3 * indices[i] as usize] as f64,
                positions[3 * indices[i] as usize + 1] as f64,
                positions[3 * indices[i] as usize + 2] as f64,
            ) * scale;
            let v1 = Point3::new(
                positions[3 * indices[i + 1] as usize] as f64,
                positions[3 * indices[i + 1] as usize + 1] as f64,
                positions[3 * indices[i + 1] as usize + 2] as f64,
            ) * scale;
            let v2 = Point3::new(
                positions[3 * indices[i + 2] as usize] as f64,
                positions[3 * indices[i + 2] as usize + 1] as f64,
                positions[3 * indices[i + 2] as usize + 2] as f64,
            ) * scale;

            let uv0 = if 2 * (indices[i] + 1) <= texcoords.len() as u32 {
                UV::new(
                    texcoords[2 * indices[i] as usize] as f64,
                    texcoords[2 * indices[i] as usize + 1] as f64,
                )
            } else {
                UV::default()
            };
            let uv1 = if 2 * (indices[i + 1] + 1) <= texcoords.len() as u32 {
                UV::new(
                    texcoords[2 * indices[i + 1] as usize] as f64,
                    texcoords[2 * indices[i + 1] as usize + 1] as f64,
                )
            } else {
                UV::default()
            };
            let uv2 = if 2 * (indices[i + 2] + 1) <= texcoords.len() as u32 {
                UV::new(
                    texcoords[2 * indices[i + 2] as usize] as f64,
                    texcoords[2 * indices[i + 2] as usize + 1] as f64,
                )
            } else {
                UV::default()
            };

            let n0 = if 3 * (indices[i] + 1) <= normals.len() as u32 {
                Point3::new(
                    normals[3 * indices[i] as usize] as f64,
                    normals[3 * indices[i] as usize + 1] as f64,
                    normals[3 * indices[i] as usize + 2] as f64,
                )
            } else {
                Point3::default()
            };
            let n1 = if 3 * (indices[i + 1] + 1) <= normals.len() as u32 {
                Point3::new(
                    normals[3 * indices[i + 1] as usize] as f64,
                    normals[3 * indices[i + 1] as usize + 1] as f64,
                    normals[3 * indices[i + 1] as usize + 2] as f64,
                )
            } else {
                Point3::default()
            };
            let n2 = if 3 * (indices[i] + 1) <= normals.len() as u32 {
                Point3::new(
                    normals[3 * indices[i + 2] as usize] as f64,
                    normals[3 * indices[i + 2] as usize + 1] as f64,
                    normals[3 * indices[i + 2] as usize + 2] as f64,
                )
            } else {
                Point3::default()
            };

            triangles.push(Triangle::new_point(
                v0,
                v1,
                v2,
                uv0,
                uv1,
                uv2,
                n0,
                n1,
                n2,
                material.clone(),
            ));
        }
    }

    triangles
}

pub fn load_model(
    obj_path: &str,
    mtl_path: &str,
    world: &mut HittableList,
    ang: f64,
    place: Vec3,
    scale: f64,
) {
    let triangles = load_obj(obj_path, mtl_path, scale);
    let mut model = HittableList::new();
    for triangle in triangles {
        model.add(Arc::new(triangle));
    }
    let bvh_model = BvhNode::new(model);
    let model = Arc::new(RotateY::new(Arc::new(bvh_model), ang));
    let model = Arc::new(Translate::new(model, place));
    world.add(model);
}
