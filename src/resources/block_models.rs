use bevy::prelude::*;

use serde::Deserialize;
use std::fs;


// Partial structs - these structs are used when parsing from json to rust
#[derive(Deserialize)]
struct PartialBlockModels {
    block_models: Vec<PartialBlockModel>
}

#[derive(Deserialize)]
struct PartialBlockModel {
    name: String,
    faces: Option<Vec<PartialFace>>,
    texture_path: Option<String>,
}

#[derive(Deserialize)]
struct PartialFace {
    position: Vec<f32>,
    rotation: Rotation,
    uv_top_left: Vec<f32>,
    uv_bottom_right: Vec<f32>,
    size: Size
}

#[derive(Deserialize)]
struct Rotation {
    yaw: f32,
    pitch: f32,
    roll: f32
}

#[derive(Deserialize)]
struct Size {
    x: f32,
    y: f32
}

// 'Full' structs -
// These are used to interface between this resource and the rest of the codebase
#[derive(Resource)]
pub struct BlockModels {
    pub block_models: Vec<BlockModel>,
    pub texture_atlas: Option<Handle<Image>>
}

#[allow(dead_code)]
pub struct BlockModel {
    pub name: String,
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub texture_path: Option<String>
}


impl From<PartialBlockModels> for BlockModels {
    fn from(partial_block_models: PartialBlockModels) -> Self {
        let mut block_models: BlockModels = BlockModels { 
            block_models: vec![],
            texture_atlas: None
        };
        for partial_block_model in partial_block_models.block_models {
            let mut vertices: Vec<[f32;3]> = Vec::with_capacity(24);
            let mut indices: Vec<u32> = Vec::with_capacity(36);
            let mut normals: Vec<[f32;3]> = Vec::with_capacity(24);
            let mut uvs: Vec<[f32;2]> = Vec::with_capacity(24);

            match partial_block_model.faces {
                Some(partial_faces) => {
                    let mut index_counter = 0;

                    for partial_face in partial_faces {
                        let position: Vec3 = Vec3::new(
                            partial_face.position[0],
                            partial_face.position[1],
                            partial_face.position[2],
                        );

                        let yaw = Quat::from_rotation_y(f32::to_radians(partial_face.rotation.yaw));
                        let pitch = Quat::from_rotation_x(f32::to_radians(partial_face.rotation.pitch));
                        let roll = Quat::from_rotation_z(f32::to_radians(partial_face.rotation.roll));

                        let normal = pitch.mul_vec3(yaw.mul_vec3(Vec3::new(0.0, 0.0, 1.0)));

                        let size_x = partial_face.size.x;
                        let size_y = partial_face.size.y;
                        
                        let bottom_left =  Vec3::new(-0.5 * size_x, -0.5 * size_y, 0.0);
                        let bottom_right = Vec3::new(0.5 * size_x,  -0.5 * size_y, 0.0);
                        let top_left =     Vec3::new(-0.5 * size_x,  0.5 * size_y, 0.0);
                        let top_right =    Vec3::new(0.5 * size_x,   0.5 * size_y, 0.0);

                        let bottom_left =  pitch.mul_vec3(yaw.mul_vec3(roll.mul_vec3(bottom_left)));
                        let bottom_right = pitch.mul_vec3(yaw.mul_vec3(roll.mul_vec3(bottom_right)));
                        let top_left =     pitch.mul_vec3(yaw.mul_vec3(roll.mul_vec3(top_left)));
                        let top_right =    pitch.mul_vec3(yaw.mul_vec3(roll.mul_vec3(top_right)));
                        
                        vertices.push((bottom_left + position).to_array());
                        vertices.push((bottom_right + position).to_array());
                        vertices.push((top_right + position).to_array());
                        vertices.push((top_left + position).to_array());

                        indices.push(index_counter);
                        indices.push(index_counter + 1);
                        indices.push(index_counter + 2);
                        indices.push(index_counter);
                        indices.push(index_counter + 2);
                        indices.push(index_counter + 3);

                        uvs.push([partial_face.uv_top_left[0], partial_face.uv_bottom_right[1]]);
                        uvs.push([partial_face.uv_bottom_right[0], partial_face.uv_bottom_right[1]]);
                        uvs.push([partial_face.uv_bottom_right[0], partial_face.uv_top_left[1]]);
                        uvs.push([partial_face.uv_top_left[0], partial_face.uv_top_left[1]]);

                        for _ in 0..4 { 
                            normals.push(normal.to_array()) 
                        }

                        index_counter += 4;
                    }
                },
                None => { 
                    
                }
            }

            let block_model: BlockModel = BlockModel {
                name: partial_block_model.name,
                vertices: vertices,
                indices: indices,
                normals: normals,
                uvs: uvs,
                texture_path: None
            };
            println!("Vertices: ");
            for vertex in &block_model.vertices {
                println!("x: {}, y: {}, z: {}", vertex[0], vertex[1], vertex[2]);
            }
            println!("Indices: ");
            for index in &block_model.indices {
                println!("{index}");
            }

            block_models.block_models.push(block_model)
        }

        return block_models
    }
}


impl Default for BlockModels {
    fn default() -> Self {
        let data = fs::read_to_string(r#"C:\Users\Ph03n\rust\block_game\src\Blocks.json"#).expect("Couldn't read file");
        let partial_block_models: PartialBlockModels =
            serde_json::from_str(&data).expect("Couldn't deserialize json file");

        return BlockModels::from(partial_block_models)
    }
}

impl BlockModels {
    pub fn get_block_model(&self, block_id: u16) -> Option<&BlockModel> {
        if (block_id as usize) < self.block_models.len() {
            return Some(&self.block_models[block_id as usize])
        }
        return None
    }
}
