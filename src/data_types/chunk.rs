// Bevy
use bevy::prelude::*;


pub struct Chunk {
    pub block_ids: [u16; 32768],
    pub position: Vec3,
}


impl Chunk {
    pub fn new(position: Vec3) -> Self {
        Self {
            block_ids: [0; 32768],
            position: position,
        }
    }

    pub fn position_from_index(index: usize) -> [f32;3] {
        return [
            (index % 32) as f32, 
            ((index / 32) % 32) as f32, 
            ((index / 1024) % 32) as f32
            ]
    }

    pub fn fill_chunk(&mut self, block_id: u16) {
        self.block_ids = [block_id; 32768]
    }
}
