use bevy::prelude::*;

#[derive(Resource)]
pub struct GameConfig {
    // Player variables
    pub player_speed_xz: f32,
    pub player_speed_y: f32,
    
    // A hidden sensitivity multiplier is used,
    // so that the visible one is a reasonable,
    // number instead of 0.00000... something
    pub hidden_sensitivity: f32,
}


impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player_speed_xz: 2.0,
            player_speed_y: 2.0,

            hidden_sensitivity: 0.001,
        }
    }
}