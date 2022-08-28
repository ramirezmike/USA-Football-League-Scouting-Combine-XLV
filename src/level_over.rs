use crate::{
    assets::GameAssets, cleanup, game_state, menus, AppState, ui::text_size, ingame, other_persons,
    component_adder::AnimationLink, game_camera, ingame_ui, title_screen::MenuAction, LEFT_GOAL, football,
    asset_loading, cutscene, assets,
};
use bevy::prelude::*;

pub struct LevelOverPlugin;
impl Plugin for LevelOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::LevelOver)
           .with_system(load_next_level)
        )
        .add_system_set(SystemSet::on_enter(AppState::LevelOver)
           .with_system(cleanup::<game_state::LevelOverCleanupMarker>)
        );
    }
}

fn load_next_level(
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_state: ResMut<game_state::GameState>,
    mut game_assets: ResMut<assets::GameAssets>,
    mut cutscene_state: ResMut<cutscene::CutsceneState>,
) {
    game_state.score = 0;
    game_state.corn_spawned = false;
    game_state.current_round += 1;

    match game_state.current_round {
        1 => cutscene_state.init(cutscene::Cutscene::LevelTwoIntro),
        _ => cutscene_state.init(cutscene::Cutscene::Intro),
    }

    assets_handler.load(AppState::InGame, &mut game_assets, &game_state);
}
