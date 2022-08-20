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

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(audio::GameAudioPlugin)
        .add_plugin(assets::AssetsPlugin)
        .add_plugin(asset_loading::AssetLoadingPlugin)
        .add_plugin(component_adder::ComponentAdderPlugin)
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
        .add_system(exit)
        .run();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Initial,
    Pause,
    Debug,
    TitleScreen,
    InGame,
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
    assets_handler.load(AppState::TitleScreen, &mut game_assets);
}

fn exit(keys: Res<Input<KeyCode>>, mut exit: ResMut<Events<AppExit>>) {
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}
