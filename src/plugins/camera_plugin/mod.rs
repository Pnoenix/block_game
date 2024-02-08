use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

use crate::settings::*;
use crate::game_settings::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (translate_camera, rotate_camera));
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    };

    commands.spawn(camera);
}

fn rotate_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut mouse: EventReader<MouseMotion>,
) {
    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse.read() {
        mouse_delta += mouse_event.delta;
    }

    let mut camera = match camera_query.get_single_mut() {
        Ok(rotation) => rotation,
        Err(_) => return
    };

    mouse_delta *= HIDDEN_SENSITIVITY * SENSITIVITY * -1.0;

    camera.rotate_y(mouse_delta.x);
    camera.rotate_local_x(mouse_delta.y);
}

fn translate_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut player = match camera_query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => return
    };

    let mut translate_xz: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let mut translate_y: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    // xz plane
    if keyboard.pressed(FORWARD) {
        translate_xz.z -= 1.0;
    }
    if keyboard.pressed(BACKWARD) {
        translate_xz.z += 1.0;
    }
    if keyboard.pressed(RIGHT) {
        translate_xz.x += 1.0;
    }
    if keyboard.pressed(LEFT) {
        translate_xz.x -= 1.0;
    }

    //y axis
    if keyboard.pressed(UP) {
        translate_y.y += 1.0;
    }
    if keyboard.pressed(DOWN) {
        translate_y.y -= 1.0;
    }

    translate_xz = Quat::from_xyzw(
        0.0, 
        player.rotation.y, 
        0.0, 
        player.rotation.w
    ).mul_vec3(translate_xz);

    match translate_xz.try_normalize() {
        Some(translate_xz) => player.translation += translate_xz * PLAYER_SPEED_XZ,
        None => { }
    }

    player.translation += translate_y * PLAYER_SPEED_Y;
}

