use crate::{
    asset_loading, assets::GameAssets, cleanup, collision, component_adder, game_camera,
    game_state, player, AppState, audio::GameAudio, component_adder::AnimationLink,
    combine, enemy, football, TOP_END, RIGHT_GOAL, LEFT_GOAL, BOTTOM_END, LEFT_END, RIGHT_END, banter, cutscene
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
              .with_system(light_sway)
              .with_system(game_camera::handle_will_camera),
        );
    }
}
fn light_sway(
    player: Query<&Transform, (With<player::Player>, Without<SpotLight>)>,
    mut query: Query<(&mut Transform, &mut SpotLight)>
) {
    for (mut transform, mut angles) in query.iter_mut() {
        let player = player.single();
        *transform = transform.looking_at(player.translation, Vec3::Y);
    }
}


#[derive(Component, Copy, Clone)]
pub struct CleanupMarker;

fn reset_ingame(
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<GameAssets>,
    mut game_state: ResMut<game_state::GameState>,
) {
    assets_handler.load(AppState::InGame, &mut game_assets, &mut game_state);
}

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<GameAssets>,
    game_state: &ResMut<game_state::GameState>,
) {
    assets_handler.add_glb(&mut game_assets.person, "models/person.glb");
    assets_handler.add_glb(&mut game_assets.person_blood, "models/person_blood.glb");
    assets_handler.add_animation(&mut game_assets.person_idle,"models/person.glb#Animation1");
    assets_handler.add_animation(&mut game_assets.person_run,"models/person.glb#Animation2");
    assets_handler.add_animation(&mut game_assets.person_dive,"models/person.glb#Animation0");
    assets_handler.add_glb(&mut game_assets.enemy, "models/enemy.glb");
    assets_handler.add_glb(&mut game_assets.combine, "models/combine.glb");
    assets_handler.add_animation(&mut game_assets.combine_drive,"models/combine.glb#Animation0");

    assets_handler.add_audio(&mut game_assets.blip, "audio/blip.wav");
    assets_handler.add_audio(&mut game_assets.touch_down, "audio/touch_down.wav");
    assets_handler.add_audio(&mut game_assets.corn_harvest, "audio/corn_harvest.wav");
    assets_handler.add_audio(&mut game_assets.dive, "audio/dive.wav");
    assets_handler.add_audio(&mut game_assets.attach, "audio/attach.wav");
    assets_handler.add_audio(&mut game_assets.will_speak, "audio/will_speak.wav");
    assets_handler.add_audio(&mut game_assets.football_launch, "audio/football_launch.wav");
    assets_handler.add_audio(&mut game_assets.tackle_sound, "audio/tackle_sound.wav");
    assets_handler.add_audio(&mut game_assets.player_death, "audio/player_death.wav");
    assets_handler.add_audio(&mut game_assets.bounce, "audio/bounce.wav");
    assets_handler.add_audio(&mut game_assets.football_pop, "audio/football_pop.wav");
    assets_handler.add_audio(&mut game_assets.bill_speak, "audio/bill_speak.wav");
    assets_handler.add_audio(&mut game_assets.bgm, "audio/combine.ogg");

    match game_state.current_round {
        1 => assets_handler.add_glb(&mut game_assets.maze, "models/maze_01.glb"),
        2 => assets_handler.add_glb(&mut game_assets.maze, "models/maze_02.glb"),
        _ => assets_handler.add_glb(&mut game_assets.maze, "models/maze.glb"),
    }

    assets_handler.add_material(&mut game_assets.bill_icon, "textures/bill.png", true);
    assets_handler.add_material(&mut game_assets.will_icon, "textures/will.png", true);

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
    game_state.attached_enemies = 0;
    game_state.enemies_spawned = false;
    game_state.touchdown_on_leftside = false;

    match game_state.current_round {
        1 => {
            // lights
            commands.insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.50,
            });
            const HALF_SIZE: f32 = 100.0;
            commands.spawn_bundle(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    // Configure the projection to better fit the scene
        //            illuminance: 10000.0,
                    illuminance: 10000.0,
                    shadow_projection: OrthographicProjection {
                        left: -HALF_SIZE,
                        right: HALF_SIZE,
                        bottom: -HALF_SIZE,
                        top: HALF_SIZE,
                        near: -10.0 * HALF_SIZE,
                        far: 10.0 * HALF_SIZE,
                        ..Default::default()
                    },
                    shadows_enabled: true,
                    ..Default::default()
                },
                transform: Transform {
                    rotation: Quat::from_rotation_x(0.80 * TAU),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(CleanupMarker);
        },
        2 => {
        },
        _ => {
            // lights
            commands.insert_resource(AmbientLight {
                color: Color::ALICE_BLUE,
                brightness: 0.02,
            });
            commands
                .spawn_bundle(SpotLightBundle {
                    transform: Transform::from_xyz(0.0, 15.0, RIGHT_GOAL)
                        .looking_at(Vec3::new(0.0, 0.0, LEFT_GOAL), Vec3::Y),
                    spot_light: SpotLight {
                        intensity: 10000.0, // lumens
                        color: Color::WHITE,
                        range: 77.0,
                        shadows_enabled: true,
                        shadow_depth_bias: 10.0,
                        inner_angle: 0.1,
                        outer_angle: 0.2,
                        ..default()
                    },
                    ..default()
                });
            commands
                .spawn_bundle(SpotLightBundle {
                    transform: Transform::from_xyz(0.0, 15.0, LEFT_GOAL)
                        .looking_at(Vec3::new(0.0, 0.0, LEFT_GOAL), Vec3::Y),
                    spot_light: SpotLight {
                        intensity: 10000.0, // lumens
                        color: Color::WHITE,
                        range: 77.0,
                        shadows_enabled: true,
                        shadow_depth_bias: 10.0,
                        inner_angle: 0.1,
                        outer_angle: 0.2,
                        ..default()
                    },
                    ..default()
                });
        }
    }

    let person_gltf = if game_state.death_count > 0 {
        assets_gltf.get(&game_assets.person_blood.clone())
    } else {
        assets_gltf.get(&game_assets.person.clone())
    };
    if let Some(gltf) = person_gltf {
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
