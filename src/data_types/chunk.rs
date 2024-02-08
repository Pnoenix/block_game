use crate::game_settings::*;
use crate::data_types::block::Block;

// Bevy
use bevy::math::Vec3;
use bevy::render::mesh::Mesh;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;


const CUBE_VERTICIES: [[f32;3];24] = [
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],

    [0.0, 0.0, -1.0],
    [0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, -1.0],

    [1.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 1.0, -1.0],
    [1.0, 1.0, -1.0],

    [1.0, 0.0, 0.0],
    [1.0, 0.0, -1.0],
    [1.0, 1.0, -1.0],
    [1.0, 1.0, 0.0],

    [0.0, 0.0, -1.0],
    [1.0, 0.0, -1.0],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 0.0],

    [0.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
    [1.0, 1.0, -1.0],
    [0.0, 1.0, -1.0],
];

const CUBE_INDICIES: [u32;36] = [
    00, 01, 02,  00, 02, 03,
    04, 05, 06,  04, 06, 07,
    08, 09, 10,  08, 10, 11,
    12, 13, 14,  12, 14, 15,
    16, 17, 18,  16, 18, 19,
    20, 21, 22,  20, 22, 23,
];

const CUBE_NORMALS: [[f32;3];24] = [
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],

    [-1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],

    [0.0, -1.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, -1.0, 0.0],

    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
];


pub struct Chunk {
    block_ids: [Block; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
    pub chunk_length: usize,
    pub position: Vec3,
}


impl Chunk {
    pub fn new(position: Vec3) -> Self {
        Self {
            block_ids: [Block::Air; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            chunk_length: CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE,
            position: position,
        }
    }

    pub fn get_block(&self, index: usize) -> Option<Block> {
        if index < self.chunk_length {
            return Some(self.block_ids[index] as Block)
        }

        return None
    }

    pub fn set_block(&mut self, block: Block, index: usize) {
        if index < self.chunk_length {
            self.block_ids[index] = block;
        }
    }

    pub fn fill_chunk(&mut self, block: Block) {
        self.block_ids = [block; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]
    }

    #[rustfmt::skip]
    pub fn generate_mesh(&self) -> Mesh {
        let mut vertex_vec: Vec<[f32;3]> = Vec::new();
        let mut index_vec: Vec<u32>      = Vec::new();
        let mut normal_vec: Vec<[f32;3]> = Vec::new();

        for chunk_index in 0..self.chunk_length {
            if self.get_block(chunk_index) == Some(Block::Air) { continue } 

            let x = (chunk_index % CHUNK_SIZE) as f32;
            let y = (chunk_index / CHUNK_SIZE % CHUNK_SIZE) as f32;
            let z = (chunk_index / CHUNK_SIZE / CHUNK_SIZE % CHUNK_SIZE) as f32;

            for vertex in CUBE_VERTICIES {
                vertex_vec.push([
                    vertex[0] + x + self.position.x * CHUNK_SIZE as f32,
                    vertex[1] + y + self.position.y * CHUNK_SIZE as f32,
                    vertex[2] + z + self.position.z * CHUNK_SIZE as f32,
                ])
            }

            for index in CUBE_INDICIES {
                match (index + (chunk_index * 24) as u32).try_into() {
                    Ok(value) => index_vec.push(value),
                    Err(_) => continue
                }
                
            }
 
            for normal in CUBE_NORMALS {
                normal_vec.push(normal)
            }
        }

        Mesh::new(PrimitiveTopology::TriangleList)

        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertex_vec,
        )
        
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normal_vec,
        )

        .with_indices(Some(Indices::U32(
            index_vec,
        )))
    }
}
