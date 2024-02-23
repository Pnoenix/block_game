use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::resources::{game_config::GameConfig, controls::Controls};


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (translate_camera, rotate_camera));
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (

        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
    );

    commands.spawn(camera);
}

fn rotate_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut mouse: EventReader<MouseMotion>,
    settings: Res<Controls>,
    game_config: Res<GameConfig>,
) {
    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse.read() {
        mouse_delta += mouse_event.delta;
    }

    let mut camera = match camera_query.get_single_mut() {
        Ok(rotation) => rotation,
        Err(_) => return
    };

    mouse_delta *= game_config.hidden_sensitivity * settings.sensitivity * -1.0;

    camera.rotate_y(mouse_delta.x);
    camera.rotate_local_x(mouse_delta.y);
}

fn translate_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<Controls>,
    game_config: Res<GameConfig>,
) {
    let mut player = match camera_query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => return
    };

    let mut translate_xz: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let mut translate_y: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    // xz plane
    if keyboard.pressed(settings.forward) {
        translate_xz.z -= 1.0;
    }
    if keyboard.pressed(settings.backward) {
        translate_xz.z += 1.0;
    }
    if keyboard.pressed(settings.right) {
        translate_xz.x += 1.0;
    }
    if keyboard.pressed(settings.left) {
        translate_xz.x -= 1.0;
    }

    //y axis
    if keyboard.pressed(settings.up) {
        translate_y.y += 1.0;
    }
    if keyboard.pressed(settings.down) {
        translate_y.y -= 1.0;
    }

    translate_xz = Quat::from_xyzw(
        0.0, 
        player.rotation.y, 
        0.0, 
        player.rotation.w
    ).mul_vec3(translate_xz);

    match translate_xz.try_normalize() {
        Some(translate_xz) => player.translation += translate_xz * game_config.player_speed_xz,
        None => { }
    }

    player.translation += translate_y * game_config.player_speed_y;
}

