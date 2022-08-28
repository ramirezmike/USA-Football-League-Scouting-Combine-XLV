#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy::asset::AssetServerSettings;
use bevy::app::AppExit;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod asset_loading;
mod assets;
mod banter;
mod audio;
mod cutscene;
mod splash;
mod billboard;
mod collision;
mod combine;
mod component_adder;
mod direction;
mod enemy;
mod football;
mod level_over;
mod game_controller;
mod game_camera;
mod game_state;
mod ingame;
mod ingame_ui;
mod maze;
mod menus;
mod player;
mod other_persons;
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
//      .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(audio::GameAudioPlugin)
        .add_plugin(assets::AssetsPlugin)
        .add_plugin(banter::BanterPlugin)
        .add_plugin(cutscene::CutscenePlugin)
        .add_plugin(asset_loading::AssetLoadingPlugin)
        .add_plugin(billboard::BillboardPlugin)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(component_adder::ComponentAdderPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(level_over::LevelOverPlugin)
        .add_plugin(football::FootballPlugin)
        .add_plugin(combine::CombinePlugin)
        .add_plugin(game_state::GameStatePlugin)
        .add_plugin(ingame_ui::InGameUIPlugin)
        .add_plugin(ingame::InGamePlugin)
        .add_plugin(maze::MazePlugin)
        .add_plugin(game_controller::GameControllerPlugin)
        .add_plugin(other_persons::OtherPersonsPlugin)
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
    Cutscene,
    Debug,
    TitleScreen,
    InGame,
    Splash,
    LevelOver,
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
    mut cutscene_state: ResMut<cutscene::CutsceneState>,
    game_state: ResMut<game_state::GameState>,
    mut banter_state: ResMut<banter::BanterState>,
    mut audio: audio::GameAudio,
) {
    // TODO: move this to title screen
    cutscene_state.init(cutscene::Cutscene::Intro);
    banter_state.reset(&game_assets);
    audio.set_volume();

    assets_handler.load(AppState::Splash, &mut game_assets, &game_state);
//    assets_handler.load(AppState::InGame, &mut game_assets, &game_state);
}

fn debug(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>, 
    game_state: ResMut<game_state::GameState>,
    mut exit: ResMut<Events<AppExit>>,
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<assets::GameAssets>,
    mut football_launch_event_writer: EventWriter<football::LaunchFootballEvent>,
    mut kill_player_event_writer: EventWriter<player::PlayerBladeEvent>,
    mut textbox_event_writer: EventWriter<ingame_ui::SetTextBoxEvent>,
    corn: Query<Entity, With<maze::CornStalk>>,
    players: Query<Entity, With<player::Player>>,
    mut cutscene_state: ResMut<cutscene::CutsceneState>,
 ) {
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }

    if keys.just_pressed(KeyCode::C) {
        for entity in &corn {
            commands.entity(entity).despawn_recursive();
        }
    }

    if keys.just_pressed(KeyCode::R) {
        assets_handler.load(AppState::ResetInGame, &mut game_assets, &game_state);
    }

    if keys.just_pressed(KeyCode::F) {
        football_launch_event_writer.send(football::LaunchFootballEvent);
    }

    if keys.just_pressed(KeyCode::E) {
        for entity in &players {
            kill_player_event_writer.send(player::PlayerBladeEvent { entity });
        }
    }

    if keys.just_pressed(KeyCode::G) {
        cutscene_state.cutscene_index = 1000;
    }

    if keys.just_pressed(KeyCode::T) {
        let texts = vec!(ingame_ui::TextBoxText {
            text: "Blah blah blah".to_string(),
            speed: 1.01,
            character: ingame_ui::DisplayCharacter::Will,
            animation_clip: game_assets.host_talk.clone(),
            after_text_displayed_delay: 1.0,
        }, ingame_ui::TextBoxText {
            text: "Ok ok ok?".to_string(),
            speed: 0.5,
            character: ingame_ui::DisplayCharacter::Bill,
            animation_clip: game_assets.host_look_left.clone(),
            after_text_displayed_delay: 1.0,
        }, ingame_ui::TextBoxText {
            text: "no no no".to_string(),
            speed: 0.5,
            after_text_displayed_delay: 1.0,
            character: ingame_ui::DisplayCharacter::Will,
            animation_clip: game_assets.host_look_left_talk.clone(),
        }, ingame_ui::TextBoxText {
            text: "yes yes yes".to_string(),
            speed: 0.5,
            after_text_displayed_delay: 1.0,
            character: ingame_ui::DisplayCharacter::Will,
            animation_clip: game_assets.host_look_right.clone(),
        }, ingame_ui::TextBoxText {
            text: "yo word word".to_string(),
            speed: 0.5,
            after_text_displayed_delay: 1.0,
            character: ingame_ui::DisplayCharacter::Bill,
            animation_clip: game_assets.host_look_right_talk.clone(),
        });

        println!("Sent texts");
        textbox_event_writer.send(ingame_ui::SetTextBoxEvent {
            texts
        });
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
