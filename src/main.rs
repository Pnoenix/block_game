// "Globals"
pub mod game_settings;
pub mod settings;
pub mod data_types;

use data_types::{chunk::*, block::*};
use game_settings::*;

// Plugins
pub mod plugins;

use plugins::camera_plugin::*;
use plugins::startup_init_plugin::*;

// Bevy
use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, StartupInitPlugin))
        .add_systems(Startup, (spawn_light, setup))
        .run();
}

fn spawn_light(mut commands: Commands) {
    let light = DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 2000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 1000.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::NEG_Z),
        ..default()
    };

    commands.spawn(light);
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let size = 16;
    for x in 0..size {
        for z in 0..size {
            let mut chunk = Chunk::new(Vec3::new(x as f32, 0.0, z as f32));
            
            for i in 0..chunk.chunk_length {

                if (i < CHUNK_SIZE * CHUNK_SIZE * 8) && (i % 3 == 0) {
                    chunk.set_block(Block::Stone, i);
                } else {
                    chunk.set_block(Block::Air, i);
                }
            }

            let cube_mesh_handle: Handle<Mesh> = meshes.add(chunk.generate_mesh());

            commands.spawn(
                PbrBundle {
                    mesh: cube_mesh_handle,
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(1.0, 0.0, 0.0),
                        ..default()
                    }),
                    ..default()
                },
            );
        }
    }
}