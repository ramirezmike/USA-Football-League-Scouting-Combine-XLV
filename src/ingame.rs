use crate::{
    asset_loading, assets::GameAssets, cleanup, collision, component_adder, game_camera,
    game_state, player, AppState, audio::GameAudio, component_adder::AnimationLink,
    combine, enemy, football, TOP_END, LEFT_GOAL, banter, cutscene
};
use std::f32::consts::{TAU, PI};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::{
    camera::RenderTarget,
    view::RenderLayers,
};

pub const RENDER_TEXTURE_SIZE: u32 = 512;
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
              .with_system(game_camera::follow_player)
              .with_system(game_camera::pan_orbit_camera)
              .with_system(game_camera::handle_will_camera),
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
    assets_handler.add_animation(&mut game_assets.person_idle,"models/person.glb#Animation1");
    assets_handler.add_animation(&mut game_assets.person_run,"models/person.glb#Animation2");
    assets_handler.add_animation(&mut game_assets.person_dive,"models/person.glb#Animation0");
    assets_handler.add_glb(&mut game_assets.enemy, "models/enemy.glb");
    assets_handler.add_glb(&mut game_assets.combine, "models/combine.glb");
    assets_handler.add_animation(&mut game_assets.combine_drive,"models/combine.glb#Animation0");
    assets_handler.add_glb(&mut game_assets.maze, "models/maze.glb");
    assets_handler.add_glb(&mut game_assets.corn_stalk, "models/corn.glb");
    assets_handler.add_glb(&mut game_assets.football, "models/football.glb");
    assets_handler.add_standard_mesh(&mut game_assets.blood_mesh, Mesh::from(shape::Plane::default()));
    assets_handler.add_animation(&mut game_assets.corn_sway,"models/corn.glb#Animation0");
    assets_handler.add_standard_material(&mut game_assets.corn_stalk_material, 
                                         StandardMaterial {
                                             unlit: true,
                                             base_color: Color::rgb(0.0, 0.5, 0.0),
                                             ..Default::default()
                                         });
    assets_handler.add_font(&mut game_assets.font, "fonts/monogram.ttf");

    assets_handler.add_glb(&mut game_assets.bill_person, "models/bill.glb");
    assets_handler.add_glb(&mut game_assets.will_person, "models/will.glb");
    assets_handler.add_animation(&mut game_assets.host_idle,"models/will.glb#Animation0");
    assets_handler.add_animation(&mut game_assets.host_look_left,"models/will.glb#Animation1");
    assets_handler.add_animation(&mut game_assets.host_look_right,"models/will.glb#Animation2");

    assets_handler.add_animation(&mut game_assets.host_talk,"models/will.glb#Animation4");
    assets_handler.add_animation(&mut game_assets.host_look_left_talk,"models/will.glb#Animation5");
    assets_handler.add_animation(&mut game_assets.host_look_right_talk,"models/will.glb#Animation6");

    assets_handler.add_material(
        &mut game_assets.blood,
        "textures/blood.png",
        true,
    );

    let size = Extent3d {
        width: RENDER_TEXTURE_SIZE,
        height: RENDER_TEXTURE_SIZE,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };
    image.resize(size);
    let image_handle = assets_handler.images.add(image); 
    game_assets.will_camera = image_handle;

    let material = assets_handler.materials.add(StandardMaterial {
        base_color_texture: Some(game_assets.will_camera.clone()),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });
    game_assets.will_material = asset_loading::GameTexture {
        material,
        image: game_assets.will_camera.clone()
    };
}

pub fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    mut game_state: ResMut<game_state::GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut component_adder: ResMut<component_adder::ComponentAdder>,
    mut audio: GameAudio,
    mut banter_state: ResMut<banter::BanterState>,
    cutscene_state: Res<cutscene::CutsceneState>,
    mut football_launch_event_writer: EventWriter<football::LaunchFootballEvent>,
    mut camera: Query<&mut Transform, With<game_camera::PanOrbitCamera>>,
) {
    println!("Setting up ingame!");

    banter_state.reset(&game_assets);
    game_state.attached_enemies = 0;
    game_state.enemies_spawned = false;
    game_state.touchdown_on_leftside = false;
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });

    if let Some(gltf) = assets_gltf.get(&game_assets.person.clone()) {
        commands.spawn_bundle(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: {
                        let mut t = Transform::from_xyz(0.0, 0.0, LEFT_GOAL);
                        t.rotate_y(TAU * 0.75);
                        t
                    },
                    ..default()
                })
                .insert_bundle(player::PlayerBundle::new())
                .insert(AnimationLink {
                    entity: None
                })
                .with_children(|parent| {
                    if let Some(football_gltf) = assets_gltf.get(&game_assets.football.clone()) {
                        parent.spawn_bundle(SceneBundle {
                                  scene: football_gltf.scenes[0].clone(),
                                  transform: {
                                      let mut t = Transform::from_scale(Vec3::splat(2.5));
                                      t.translation.y += 1.0;
                                      t.translation.x += 0.5;
                                      t.rotation = Quat::from_rotation_z(TAU * 0.75);

                                      t
                                  },
                                  visibility: Visibility { is_visible: false },
                                  ..default()
                              })
                              .insert(football::CarriedFootball);
                    }
                })
                .insert(CleanupMarker);
    }

    if let Some(gltf) = assets_gltf.get(&game_assets.enemy.clone()) {
        // kickers
        commands.spawn_bundle(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: {
                        let mut t = Transform::from_xyz(6.976, 0.0, -48.0);
                        t.rotation = Quat::from_rotation_y(TAU * 0.75);
                        t
                    },
                    ..default()
                })
                .insert(CleanupMarker);
        commands.spawn_bundle(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: {
                        let mut t = Transform::from_xyz(6.976, 0.0, 48.0);
                        t.rotation = Quat::from_rotation_y(TAU * 0.25);
                        t
                    },
                    ..default()
                })
                .insert(CleanupMarker);
    }

    if let Some(gltf) = assets_gltf.get(&game_assets.combine.clone()) {
        commands.spawn_bundle(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: {
                        let mut t = Transform::from_xyz(TOP_END * 0.5, 0.0, (game_state.maze_size / 2.0));
                        t.rotate_y(TAU * 0.25);
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

    if cutscene_state.current.is_none() {
        if camera.iter().len() == 0 {
            game_camera::spawn_camera(&mut commands, CleanupMarker, &game_assets,
                                      Vec3::new(game_camera::INGAME_CAMERA_X, 
                                               game_camera::INGAME_CAMERA_Y, 
                                               LEFT_GOAL),
                                  Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                            game_camera::INGAME_CAMERA_ROTATION_ANGLE));
        } else {
            for mut camera in &mut camera {
                camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                               game_camera::INGAME_CAMERA_Y, 
                                               LEFT_GOAL);
                camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                            game_camera::INGAME_CAMERA_ROTATION_ANGLE);
            }
        }

        football_launch_event_writer.send(football::LaunchFootballEvent);
    } else if camera.iter().len() == 0 {
        game_camera::spawn_camera(&mut commands, CleanupMarker, &game_assets,
                                  Vec3::new(22.5, 1.5, 0.0),
                            Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247));
    }
}
