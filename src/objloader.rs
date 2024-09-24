use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::venom_core::*;

impl Scene {
    pub fn new() -> Self {
        Self { models: Vec::new() }
    }

    pub fn load_obj(file_path: &str) -> Result<Self, String> {
        let file = File::open(file_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        let mut positions: Vec<(f32, f32, f32)> = Vec::new();
        let mut normals: Vec<(f32, f32, f32)> = Vec::new();
        let mut uvs: Vec<(f32, f32)> = Vec::new();

        let mut current_mesh = Mesh { vertices: Vec::new() };
        let mut current_model = Model {
            meshes: Vec::new(),
            name: String::new(),
        };
        let mut scene = Scene::new();

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" => {
                    let x: f32 = parts[1].parse::<f32>().unwrap();
                    let y: f32 = parts[2].parse::<f32>().unwrap();
                    let z: f32 = parts[3].parse::<f32>().unwrap();
                    positions.push((x, y, z));
                }
                "vn" => {
                    let x: f32 = parts[1].parse::<f32>().unwrap();
                    let y: f32 = parts[2].parse::<f32>().unwrap();
                    let z: f32 = parts[3].parse::<f32>().unwrap();
                    normals.push((x, y, z));
                }
                "vt" => {
                    let u: f32 = parts[1].parse::<f32>().unwrap();
                    let v: f32 = parts[2].parse::<f32>().unwrap();
                    uvs.push((u, v));
                }
                "f" => {
                    for face_part in &parts[1..] {
                        let indices: Vec<&str> = face_part.split('/').collect();
                        let pos_idx: usize = indices[0].parse::<usize>().map_err(|e| e.to_string())? - 1;
                        let uv_idx: usize = indices[1].parse::<usize>().map_err(|e| e.to_string())? - 1;
                        let norm_idx: usize = indices[2].parse::<usize>().map_err(|e| e.to_string())? - 1;

                        if pos_idx < positions.len() && uv_idx < uvs.len() && norm_idx < normals.len() {
                            let vertex = Vertex {
                                position: positions[pos_idx],
                                uv: uvs[uv_idx],
                                normal: normals[norm_idx],
                            };
                            current_mesh.vertices.push(vertex);
                        } else {
                            return Err("Face references out of range vertex, normal, or uv index".to_string());
                        }
                    }
                }
                "o" => {
                    if !current_model.meshes.is_empty() {
                        scene.models.push(current_model.clone());
                        current_model = Model {
                            meshes: Vec::new(),
                            name: String::new(),
                        };
                    }
                    current_model.name = parts[1].to_string();
                }
                "g" => {
                    if !current_mesh.vertices.is_empty() {
                        current_model.meshes.push(current_mesh.clone());
                        current_mesh = Mesh { vertices: Vec::new() };
                    }
                }
                _ => {}
            }
        }

        if !current_mesh.vertices.is_empty() {
            current_model.meshes.push(current_mesh);
        }

        if !current_model.meshes.is_empty() {
            scene.models.push(current_model);
        }

        Ok(scene)
    }
}
