use crate::material::texture::MappedTexture;
use crate::material::texture::rtw_stb_image::RtwImage;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::ToSocketAddrs;
use std::ptr::null_mut;

pub struct MtlInfo {
    pub name: String,
    pub map_kd: Option<String>,
    pub map_bump: Option<String>,
    pub map_d: Option<String>,
}

pub fn process_mtl_file(path: &str) -> HashMap<String, MtlInfo> {
    let file = File::open(path).expect("Cannot open mtl file");
    let reader = BufReader::new(file);

    let mut res = HashMap::new();
    let mut now = MtlInfo {
        name: String::new(),
        map_kd: None,
        map_bump: None,
        map_d: None,
    };

    for line in reader.lines().map_while(Result::ok) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens.first().copied() {
            Some("newmtl") => {
                if !now.name.is_empty() {
                    res.insert(now.name.clone(), now);
                }
                now = MtlInfo {
                    name: tokens[1].to_string(),
                    map_kd: None,
                    map_bump: None,
                    map_d: None,
                };
            }
            Some("map_Kd") => {
                now.map_kd = tokens.get(1).map(|s| s.to_string());
            }
            Some("map_bump") => {
                now.map_bump = tokens.get(1).map(|s| s.to_string());
            }
            Some("bump") => {
                now.map_bump = tokens.get(1).map(|s| s.to_string());
            }
            Some("map_d") => {
                now.map_d = tokens.get(1).map(|s| s.to_string());
            }
            _ => {}
        }
    }

    if !now.name.is_empty() {
        res.insert(now.name.clone(), now);
    }

    res
}

pub fn get_mapped_texture(material: &MtlInfo) -> MappedTexture {
    MappedTexture::new(
        material
            .map_kd
            .as_deref()
            .unwrap_or("textures/diffuse_default.jpg"),
        material.map_bump.as_deref(),
        material.map_d.as_deref(),
    )
}
