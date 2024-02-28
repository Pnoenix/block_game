// "Globals"
pub mod resources;
pub mod data_types;

use resources::{game_config::*, controls::*, block_models::*};

// Plugins
pub mod plugins;

use plugins::{camera_plugin::*, startup_init_plugin::*, chunk_queue::*};

// Bevy
use bevy::prelude::*;

// Own
use data_types::chunk::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, 
            CameraPlugin, 
            StartupInitPlugin, 
            ChunkQueue
        ))
        .add_systems(Startup, (spawn_light, chunk_queue_test))
        .init_resource::<Controls>()
        .init_resource::<GameConfig>()
        .init_resource::<BlockModels>()
        .run();
}


fn spawn_light(mut commands: Commands) {
    let light = DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 1000.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::NEG_Z),
        ..default()
    };

    commands.spawn(light);
}

fn chunk_queue_test(mut chunk_load_queue: ResMut<ChunkLoadQueue>) {
    let mut chunk: Chunk = Chunk::new(Vec3::new(0.0, 0.0, 0.0));
    chunk.fill_chunk(1);

    chunk_load_queue.0.push(chunk);
}