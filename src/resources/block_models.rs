use bevy::prelude::*;

use serde_json::Result;
use serde::Deserialize;

use std::fs;

pub enum Direction {
    Forward,
    Backward,
    Right,
    Left,
    Up,
    Down,
}


#[derive(Resource, Deserialize)]
pub struct BlockModels {
    block_models: Vec<BlockModel>
}


#[derive(Deserialize)]
pub struct BlockModel {
    faces: Vec<Face>,
    uvs: Vec<[u32; 2]>,
    texture_path: String,
}


#[derive(Deserialize)]
pub struct Face {
    position: [f32; 3],
    rotation: [f32; 3],
    size: f32,
}


impl Default for BlockModels {
    fn default() -> Self {
        let data = fs::read_to_string(r#"C:\Users\Ph03n\rust\block_game\src\Blocks.json"#).expect("Couldn't read block model file");

        Self {
            block_models: serde_json::from_str(&data).expect("Couldn't parse block models file"),
        }
    }
}