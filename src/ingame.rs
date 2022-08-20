use crate::{
    asset_loading, assets::GameAssets, cleanup, collision, component_adder, game_camera,
    game_state, player, AppState, audio::GameAudio, component_adder::AnimationLink,
    combine,
};
use bevy::gltf::Gltf;
use bevy::prelude::*;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(game_camera::spawn_camera)
                .with_system(setup),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(game_camera::pan_orbit_camera),
        );
    }
}

#[derive(Component)]
struct CleanupMarker;

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<GameAssets>,
) {
    assets_handler.add_glb(&mut game_assets.person, "models/person.glb");
    assets_handler.add_animation(&mut game_assets.person_idle,"models/person.glb#Animation0");
    assets_handler.add_animation(&mut game_assets.person_run,"models/person.glb#Animation1");
    assets_handler.add_glb(&mut game_assets.maze, "models/maze.glb");
    assets_handler.add_standard_mesh(&mut game_assets.corn_stalk, Mesh::from(shape::Cube::new(0.1)));
    assets_handler.add_standard_material(&mut game_assets.corn_stalk_material, 
                                         StandardMaterial {
                                             unlit: true,
                                             base_color: Color::rgb(0.0, 0.5, 0.0),
                                             ..Default::default()
                                         });
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    game_state: Res<game_state::GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut component_adder: ResMut<component_adder::ComponentAdder>,
    mut audio: GameAudio,
) {
    println!("Setting up ingame");
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });

    if let Some(gltf) = assets_gltf.get(&game_assets.person.clone()) {
        commands.spawn_bundle(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    ..default()
                })
                .insert_bundle(player::PlayerBundle::new())
                .insert(combine::Combine)
                .insert(AnimationLink {
                    entity: None
                })
                .insert(CleanupMarker);
    }

    if let Some(gltf) = assets_gltf.get(&game_assets.maze.clone()) {
        commands.spawn_bundle(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    ..default()
                })
                .insert(CleanupMarker);
    }

    component_adder.reset();

    if game_state.music_on {
//        audio.play_bgm(&game_assets.game_music);
    } else {
        audio.stop_bgm();
    }
}
