use crate::{AppState};
use bevy::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default())
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(placeholder)
            );
    }
}

pub struct GameState {
    pub score: usize,
    pub music_on: bool 
}

impl GameState {
    pub fn initialize(music_on: bool) -> Self {
        GameState {
            score: 0,
            music_on: music_on
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            score: 0,
            music_on: true,
        }
    }
}

fn placeholder() {}
