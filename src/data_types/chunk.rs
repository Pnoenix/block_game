use crate::data_types::block::Block;

// Bevy
use bevy::math::Vec3;
use bevy::render::mesh::Mesh;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;


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
    block_ids: [Block; 32 * 32 * 32],
    pub chunk_length: usize,
    pub position: Vec3,
}


impl Chunk {
    pub fn new(position: Vec3) -> Self {
        Self {
            block_ids: [Block::Air; 32 * 32 * 32],
            chunk_length: 32 * 32 * 32,
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
        self.block_ids = [block; 32 * 32 * 32]
    }

    #[rustfmt::skip]
    pub fn generate_mesh(&self) -> Mesh {
        let mut vertex_vec: Vec<[f32;3]> = Vec::new();
        let mut index_vec: Vec<u32>      = Vec::new();
        let mut normal_vec: Vec<[f32;3]> = Vec::new();

        let mut vertex_index: u32 = 0;

        for chunk_index in 0..self.chunk_length {
            if self.get_block(chunk_index) == Some(Block::Air) { continue } 

            let x = (chunk_index % 32) as f32;
            let y = (chunk_index / 32 / 32 % 32) as f32;
            let z = (chunk_index / 32 % 32) as f32;

            for vertex in CUBE_VERTICIES {
                vertex_vec.push([
                    vertex[0] + x + self.position.x * 32 as f32,
                    vertex[1] + y + self.position.y * 32 as f32,
                    vertex[2] + z + self.position.z * 32 as f32,
                ])
            }

            for index in CUBE_INDICIES {
                match (index + (vertex_index * 24) as u32).try_into() {
                    Ok(value) => index_vec.push(value),
                    Err(_) => continue
                }
            }
 
            for normal in CUBE_NORMALS {
                normal_vec.push(normal)
            }

            vertex_index += 1;
        }

        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)

        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertex_vec,
        )
        
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normal_vec,
        )

        .with_inserted_indices(Indices::U32(
            index_vec,
        ))
    }
}
