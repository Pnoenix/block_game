use bevy::prelude::*;
use crate::data_types::{chunk::*, marker::ChunkMarker};
use crate::resources::block_models::*;

use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;


pub struct ChunkQueue;

impl Plugin for ChunkQueue {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ChunkLoadQueue>()
            .init_resource::<ChunkUnloadQueue>()
            .add_systems(Update, (load_chunks, unload_chunks));
    }
}


fn load_chunks(
    mut commands: Commands,
    chunk_load_queue: Res<ChunkLoadQueue>,
    block_models: Res<BlockModels>,

    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    if chunk_load_queue.0.len() == 0 { return }

    let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(24);
    let mut indices: Vec<u32> = Vec::with_capacity(36);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(24);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(24);

    let chunk = chunk_load_queue.0.first().expect("This shouldn't be possible...?");

    let mut index_offset: u32 = 0;

    for block_id in chunk.block_ids {
        let block_model = block_models.get_block_model(block_id).expect("Couldn't find block");

        vertices.append(&mut block_model.vertices.clone());
        normals.append(&mut block_model.normals.clone());
        uvs.append(&mut block_model.uvs.clone());
        indices.append(&mut block_model.indices.clone().iter().map(|x|x + index_offset).collect());

        index_offset += block_model.indices.len() as u32;
    }

    let block_mesh = meshes.add(
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            uvs
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals
        )
        .with_inserted_indices(Indices::U32(
            indices
        )
    ));

    commands.spawn(
        PbrBundle {
            mesh: block_mesh,
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                ..default()
            }),
            ..default()
        }
    );
}


fn unload_chunks(
    mut commands: Commands,
    chunk_unload_queue: Res<ChunkUnloadQueue>,
    chunks_query: Query<(Entity, &ChunkMarker)>
) {
    if chunk_unload_queue.0.len() == 0 { return }
    
    for despawn_position in &chunk_unload_queue.0 {
        for (entity, chunk_marker) in chunks_query.iter() {
            if &chunk_marker.0 == despawn_position {
                commands.entity(entity).despawn();
            }
        }
    }
}


#[derive(Resource)]
pub struct ChunkLoadQueue(Vec<Chunk>);

#[derive(Resource)]
pub struct ChunkUnloadQueue(Vec<Vec3>);

impl Default for ChunkLoadQueue {
    fn default() -> Self {
        ChunkLoadQueue(vec![])
    }
}

impl Default for ChunkUnloadQueue {
    fn default() -> Self {
        ChunkUnloadQueue(vec![])
    }
}