use bevy::prelude::*;
use bevy::window::{WindowMode, CursorGrabMode, PrimaryWindow};

pub struct StartupInitPlugin;

impl Plugin for StartupInitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (grab_cursor, make_fullscreen, set_app_name));
    }
}

fn grab_cursor(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::Confined;
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;

    primary_window.cursor.visible = false;
}

fn make_fullscreen(
    mut windows: Query<&mut Window>
) {
    let mut window = windows.get_single_mut().expect("yes");

    window.mode = WindowMode::Fullscreen;
}

fn set_app_name(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.title = "I no no wanna".to_string();
    } 
}