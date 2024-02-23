use bevy::prelude::*;


#[derive(Resource)]
pub struct Controls {
    // Mouse
    pub sensitivity: f32,

    // Keyboard
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub right: KeyCode,
    pub left: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
}


impl Default for Controls {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,

            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            right: KeyCode::KeyD,
            left: KeyCode::KeyA,
            up: KeyCode::Space,
            down: KeyCode::ControlLeft,
        }
    }
}