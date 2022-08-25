#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy::asset::AssetServerSettings;
use bevy::app::AppExit;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod asset_loading;
mod assets;
mod audio;
mod collision;
mod combine;
mod component_adder;
mod direction;
mod enemy;
mod football;
mod game_controller;
mod game_camera;
mod game_state;
mod ingame;
mod maze;
mod menus;
mod player;
mod title_screen;
mod shaders;
mod ui;

const LEFT_GOAL:f32 = -38.5;
const RIGHT_GOAL:f32 = 37.5;
const LEFT_END:f32 = -47.5;
const RIGHT_END:f32 = 47.0;
const BOTTOM_END:f32 = -19.471;
const TOP_END:f32 = 20.471;


fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
//      .add_plugin(LogDiagnosticsPlugin::default())
//      .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(audio::GameAudioPlugin)
        .add_plugin(assets::AssetsPlugin)
        .add_plugin(asset_loading::AssetLoadingPlugin)
        .add_plugin(component_adder::ComponentAdderPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(football::FootballPlugin)
        .add_plugin(combine::CombinePlugin)
        .add_plugin(game_state::GameStatePlugin)
//      .add_plugin(ingame_ui::InGameUIPlugin)
        .add_plugin(ingame::InGamePlugin)
        .add_plugin(maze::MazePlugin)
        .add_plugin(game_controller::GameControllerPlugin)
        .add_plugin(shaders::ShadersPlugin)
        .add_plugin(title_screen::TitlePlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(ui::text_size::TextSizePlugin)
        .add_state(AppState::Initial)
        .add_system_set(SystemSet::on_update(AppState::Initial).with_system(bootstrap))
        .add_system(debug)
        .run();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Initial,
    Pause,
    Debug,
    TitleScreen,
    InGame,
    ResetInGame,
    Loading,
}

pub fn cleanup<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn bootstrap(
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<assets::GameAssets>,
) {
    assets_handler.load(AppState::InGame, &mut game_assets);
}

fn debug(
    keys: Res<Input<KeyCode>>, 
    mut exit: ResMut<Events<AppExit>>,
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<assets::GameAssets>,
    mut football_launch_event_writer: EventWriter<football::LaunchFootballEvent>,
 ) {
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }

    if keys.just_pressed(KeyCode::R) {
        assets_handler.load(AppState::ResetInGame, &mut game_assets);
    }

    if keys.just_pressed(KeyCode::F) {
        football_launch_event_writer.send(football::LaunchFootballEvent);
    }
}

pub trait ZeroSignum {
    fn zero_signum(&self) -> Vec3;
}

impl ZeroSignum for Vec3 {
    fn zero_signum(&self) -> Vec3 {
        let convert = |n| {
            if n < 0.1 && n > -0.1 {
                0.0
            } else if n > 0.0 {
                1.0
            } else {
                -1.0
            }
        };

        Vec3::new(convert(self.x), convert(self.y), convert(self.z))
    }
}

