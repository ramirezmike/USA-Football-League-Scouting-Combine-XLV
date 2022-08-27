use crate::{AppState, football, player};
use bevy::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default())
            .add_event::<TouchdownEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(handle_touchdown_event.after(player::check_for_touchdown))
                    .with_system(placeholder)
            );
    }
}

#[derive(Component)]
pub struct LevelOverCleanupMarker;

pub struct GameState {
    pub score: usize,
    pub music_on: bool,
    pub maze_size: f32,
    pub touchdown_on_leftside: bool,
    pub attached_enemies: usize,
    pub enemies_spawned: bool,
    pub corn_spawned: bool,
    pub death_count: usize,
}

impl GameState {
    pub fn initialize(music_on: bool) -> Self {
        GameState {
            score: 0,
            music_on: music_on,
            attached_enemies: 0, 
            maze_size: 80.0,
            touchdown_on_leftside: false,
            corn_spawned: false,
            enemies_spawned: false,
            death_count: 0,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            score: 0,
            music_on: true,
            attached_enemies: 0, 
            enemies_spawned: false,
            maze_size: 80.0,
            corn_spawned: false,
            death_count: 0,
            touchdown_on_leftside: false,
        }
    }
}

pub struct TouchdownEvent;

fn handle_touchdown_event(
    mut touchdown_event_reader: EventReader<TouchdownEvent>,
    mut game_state: ResMut<GameState>,
    mut football_launch_event_writer: EventWriter<football::LaunchFootballEvent>,
) {
    for event in touchdown_event_reader.iter() {
        game_state.score += 100;
        game_state.touchdown_on_leftside = !game_state.touchdown_on_leftside;
        football_launch_event_writer.send(football::LaunchFootballEvent);
    }
}

fn placeholder() {}
