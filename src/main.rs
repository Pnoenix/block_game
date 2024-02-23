// "Globals"
pub mod resources;
pub mod data_types;

use data_types::{chunk::*, block::*};
use resources::{game_config::*, controls::*, block_models::*};

// Plugins
pub mod plugins;

use plugins::camera_plugin::*;
use plugins::startup_init_plugin::*;

// Bevy
use bevy::prelude::*;
use bevy::diagnostic::LogDiagnosticsPlugin;

// Other
use libnoise::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, StartupInitPlugin, LogDiagnosticsPlugin::default()))
        .add_systems(Startup, (spawn_light, setup))
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

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let size = 2;

    let simplex = Source::simplex(1);
    let mut count: i64 = 0;

    let scale = 50.0;
    
    for x in 0..size {
        for z in 0..size {
            let mut chunk = Chunk::new(Vec3::new(x as f32, 0.0, z as f32));
            
            for i in 0..chunk.chunk_length {
                if simplex.sample([(x*32 + i % 32) as f64 / scale, ((i/1024) % 32) as f64 / scale, (z*32 + (i / 32) % 32) as f64 / scale]) > 0.0 {
                    chunk.set_block(Block::Stone, i);
                    count += 12;
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

    println!("Triangle count: {count}");
}