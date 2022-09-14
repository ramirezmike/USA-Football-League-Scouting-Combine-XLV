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
    pub shadows_on: bool,
    pub graphics_high: bool, 
    pub maze_size: f32,
    pub is_latest: bool,
    pub touchdown_on_leftside: bool,
    pub attached_enemies: usize,
    pub title_screen_cooldown: f32,
    pub enemies_spawned: bool,
    pub corn_spawned: bool,
    pub death_count: usize,
    pub current_round: usize
}

impl GameState {
    pub fn initialize(graphics: bool, shadows_on: bool, game_version: bool) -> Self {
        GameState {
            score: 0,
            shadows_on: shadows_on, 
            graphics_high: graphics, 
            attached_enemies: 0, 
            is_latest: game_version, 
            maze_size: 80.0,
            touchdown_on_leftside: false,
            corn_spawned: false,
            title_screen_cooldown: 1.0,
            enemies_spawned: false,
            current_round: 0,
            death_count: 0,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            score: 0,
            shadows_on: true,
            graphics_high: true, 
            attached_enemies: 0, 
            enemies_spawned: false,
            is_latest: true,
            maze_size: 80.0,
            corn_spawned: false,
            title_screen_cooldown: 1.0,
            death_count: 0,
            current_round: 0,
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
