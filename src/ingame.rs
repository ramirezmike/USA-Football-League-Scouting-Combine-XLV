use crate::{
    asset_loading, assets::GameAssets, cleanup, collision, component_adder, game_camera,
    game_state, player, AppState, audio::GameAudio, component_adder::AnimationLink,
    combine, enemy,
};
use std::f32::consts::{TAU, PI};
use bevy::gltf::Gltf;
use bevy::prelude::*;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
                .with_system(cleanup::<CleanupMarker>)
        )
        .add_system_set(
            SystemSet::on_update(AppState::ResetInGame)
                .with_system(reset_ingame)
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(game_camera::pan_orbit_camera),
        );
    }
}

#[derive(Component, Copy, Clone)]
pub struct CleanupMarker;

fn reset_ingame(
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<GameAssets>,
) {
    assets_handler.load(AppState::InGame, &mut game_assets);
}

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<GameAssets>,
) {
    assets_handler.add_glb(&mut game_assets.person, "models/person.glb");
    assets_handler.add_animation(&mut game_assets.person_idle,"models/person.glb#Animation0");
    assets_handler.add_animation(&mut game_assets.person_run,"models/person.glb#Animation1");
    assets_handler.add_glb(&mut game_assets.combine, "models/combine.glb");
    assets_handler.add_animation(&mut game_assets.combine_drive,"models/combine.glb#Animation0");
    assets_handler.add_glb(&mut game_assets.maze, "models/maze.glb");
    assets_handler.add_glb(&mut game_assets.corn_stalk, "models/corn.glb");
    assets_handler.add_animation(&mut game_assets.corn_sway,"models/corn.glb#Animation0");
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

    game_camera::spawn_camera(&mut commands, CleanupMarker);
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });

    if let Some(gltf) = assets_gltf.get(&game_assets.person.clone()) {
        let line_of_sight_id = commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::default())),
                material: materials.add(StandardMaterial {
                    unlit: true,
                    base_color: Color::rgba(0.0, 0.0, 1.0, 0.6),
                    alpha_mode: AlphaMode::Blend,
                    ..Default::default()
                }),
                transform: Transform::from_scale(Vec3::ZERO),
                ..Default::default()
            })
            .insert(enemy::EnemyLineOfSight { })
            .id();
        commands.spawn_bundle(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    ..default()
                })
                .insert_bundle(player::PlayerBundle::new())
                .insert(AnimationLink {
                    entity: None
                })
                .insert(CleanupMarker);
    }

//  if let Some(gltf) = assets_gltf.get(&game_assets.person.clone()) {
//      let line_of_sight_id = commands
//          .spawn_bundle(PbrBundle {
//              mesh: meshes.add(Mesh::from(shape::Box::default())),
//              material: materials.add(StandardMaterial {
//                  unlit: true,
//                  base_color: Color::rgba(1.0, 0.0, 0.0, 0.6),
//                  alpha_mode: AlphaMode::Blend,
//                  ..Default::default()
//              }),
//              transform: Transform::from_scale(Vec3::ZERO),
//              ..Default::default()
//          })
//          .insert(enemy::EnemyLineOfSight { })
//          .insert(CleanupMarker)
//          .id();
//      commands.spawn_bundle(SceneBundle {
//                  scene: gltf.scenes[0].clone(),
//                  transform: Transform::from_xyz(3.0, 0.0, 2.0),
//                  ..default()
//              })
//              .insert(enemy::Enemy::new(line_of_sight_id))
//              .insert(AnimationLink {
//                  entity: None
//              })
//              .insert(CleanupMarker);
//  }

    if let Some(gltf) = assets_gltf.get(&game_assets.combine.clone()) {
        commands.spawn_bundle(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: {
                        let mut t = Transform::from_xyz(0.0, 0.0, -(game_state.maze_size / 2.0));
                        t.rotate_y(TAU * 0.75);
                        t
                    },

                    ..default()
                })
                .insert(combine::Combine::default())
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
