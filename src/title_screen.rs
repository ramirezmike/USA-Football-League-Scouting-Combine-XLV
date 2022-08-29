use crate::{
    asset_loading, assets::GameAssets, cleanup, game_controller, AppState, shaders,
    audio::GameAudio, menus, ui::text_size, game_state, cutscene, banter,
};
use bevy::app::AppExit;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::ecs::event::Events;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<MenuAction>::default())
            .add_system_set(SystemSet::on_enter(AppState::TitleScreen).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::TitleScreen)
                    .with_system(update_menu_buttons.after(handle_controllers))
                    .with_system(handle_controllers.after(game_controller::store_controller_inputs))
            )
            .add_system_set(
                SystemSet::on_exit(AppState::TitleScreen).with_system(cleanup::<CleanupMarker>),
            );
    }
}

#[derive(Component)]
pub struct CleanupMarker;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MenuAction {
    Up,
    Down,
    Left,
    Right,
    Select,
}
impl MenuAction {
    pub fn default_input_map() -> InputMap<MenuAction> {
        use MenuAction::*;
        let mut input_map = InputMap::default();

        input_map.set_gamepad(Gamepad { id: 0 });

        input_map.insert(KeyCode::Up, Up);
        input_map.insert(KeyCode::W, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::Down, Down);
        input_map.insert(KeyCode::S, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(KeyCode::A, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(KeyCode::D, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        input_map.insert(KeyCode::Space, Select);
        input_map.insert(KeyCode::Return, Select);
        input_map.insert(GamepadButtonType::South, Select);

        input_map
    }
}

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<GameAssets>,
) {
    assets_handler.add_audio(&mut game_assets.titlescreen, "audio/football.ogg");
    assets_handler.add_audio(&mut game_assets.blip, "audio/blip.wav");
    assets_handler.add_font(&mut game_assets.font, "fonts/monogram.ttf");
    assets_handler.add_material(
        &mut game_assets.title_screen_logo,
        "textures/logo.png",
        true,
    );
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut audio: GameAudio,
    mut texture_materials: ResMut<Assets<shaders::TextureMaterial>>,
    mut clear_color: ResMut<ClearColor>,
    mut banter_state: ResMut<banter::BanterState>,
    mut cutscene_state: ResMut<cutscene::CutsceneState>,
    text_scaler: text_size::TextScaler,
) {
    cutscene_state.init(cutscene::Cutscene::Intro);
    banter_state.reset(&game_assets);

    clear_color.0 = Color::hex("00068a").unwrap(); 
    let image_height = 1280.0;
    let scale = (text_scaler.window_size.height * 0.8) / image_height;
    commands.spawn_bundle(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        camera: Camera {
            priority: 1,
            ..default()
        },
        ..default()
    })
    .insert(CleanupMarker);
      commands.spawn_bundle(SpriteBundle {
          transform: {
              let height = (text_scaler.window_size.height / 2.0) * 0.224;
              let mut t = Transform::from_translation(Vec3::new(0.0, height, 0.0));
              t.apply_non_uniform_scale(Vec3::new(scale, scale, scale));
              t
          },
          texture: game_assets.title_screen_logo.image.clone(),
          ..Default::default()
        })
        .insert(CleanupMarker);

    commands
        .spawn_bundle(InputManagerBundle {
            input_map: MenuAction::default_input_map(),
            action_state: ActionState::default(),
        })
        .insert(CleanupMarker);

//  commands
//      .spawn_bundle(MaterialMeshBundle {
//          transform: {
//              let mut transform = Transform::from_scale(Vec3::splat(10.0));
//              transform.translation.z = 0.1;

//              transform
//          },
//          material: texture_materials.add(shaders::TextureMaterial {
//              env_texture: Some(game_assets.title_screen_logo.image.clone()),
//              color: Color::rgb(1.0, 1.0, 1.0),
//              time: 0.0,
//              x_scroll_speed: 0.1,
//              y_scroll_speed: 0.4,
//              scale: 0.2,
//          }),
//          mesh: meshes.add(Mesh::from(shape::Plane::default())),
//          ..Default::default()
//      })
//      .insert(CleanupMarker);

//  commands
//      .spawn_bundle(MaterialMeshBundle {
//          transform: {
//              let mut transform = Transform::from_scale(Vec3::splat(12.0));
//              transform.translation.z = 0.4;

//              transform
//          },
//          material: texture_materials.add(shaders::TextureMaterial {
//              env_texture: Some(game_assets.title_screen_logo.image.clone()),
//              color: Color::rgb(1.0, 1.0, 1.0),
//              time: 0.0,
//              x_scroll_speed: 0.2,
//              y_scroll_speed: 0.5,
//              scale: 0.3,
//          }),
//          mesh: meshes.add(Mesh::from(shape::Plane::default())),
//          ..Default::default()
//      })
//      .insert(CleanupMarker);


    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, -0.0001).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CleanupMarker);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::from_section(
                "by michael ramirez".to_string(),
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: text_scaler.scale(menus::BY_LINE_FONT_SIZE),
                    color: Color::WHITE,
                }
            ),
            ..Default::default()
        })
        .insert(CleanupMarker);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(30.0), Val::Percent(25.0)),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Percent(60.0),
                    ..Default::default()
                },
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        margin: UiRect::all(Val::Auto),
                        size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: menus::NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "Start",
                            TextStyle {
                                font: game_assets.font.clone(),
                                font_size: text_scaler.scale(menus::BUTTON_LABEL_FONT_SIZE),
                                color: Color::WHITE,
                            }
                        ),
                        ..Default::default()
                    });
                })
                .insert(CleanupMarker);

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        ..Default::default()
                    },
                    color: menus::NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "Quit",
                            TextStyle {
                                font: game_assets.font.clone(),
                                font_size: text_scaler.scale(menus::BUTTON_LABEL_FONT_SIZE),
                                color: Color::WHITE,
                            }
                        ),
                        ..Default::default()
                    });
                })
                .insert(CleanupMarker);
        });

    audio.play_bgm_once(&game_assets.titlescreen);
}

fn update_menu_buttons(
    mut selected_button: Local<usize>,
    mut exit: ResMut<Events<AppExit>>,
    buttons: Query<Entity, With<Button>>,
    mut button_colors: Query<&mut UiColor, With<Button>>,
    action_state: Query<&ActionState<MenuAction>>,
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<GameAssets>,
    mut audio: GameAudio,
    mut game_state: ResMut<game_state::GameState>,
    time: Res<Time>,
) {
    game_state.title_screen_cooldown -= time.delta_seconds();
    game_state.title_screen_cooldown = game_state.title_screen_cooldown.clamp(-3.0, 30.0);

    if game_state.title_screen_cooldown > 0.0 {
        return;
    }

    let action_state = action_state.single();
    let number_of_buttons = buttons.iter().count();
    let mut pressed_button = action_state.pressed(MenuAction::Select);

    if action_state.just_pressed(MenuAction::Up) {
        audio.play_sfx(&game_assets.blip);
        *selected_button = selected_button
            .checked_sub(1)
            .unwrap_or(number_of_buttons - 1);
    }
    if action_state.just_pressed(MenuAction::Down) {
        audio.play_sfx(&game_assets.blip);
        let new_selected_button = selected_button.checked_add(1).unwrap_or(0);
        *selected_button = if new_selected_button > number_of_buttons - 1 {
            0
        } else {
            new_selected_button
        };
    }

    for (i, mut color) in button_colors.iter_mut().enumerate() {
        if i == *selected_button {
            *color = menus::HOVERED_BUTTON.into();
        } else {
            *color = menus::NORMAL_BUTTON.into();
        }
    }

    if pressed_button {
        if *selected_button == 0 {
            audio.play_sfx(&game_assets.blip);
            assets_handler.load(AppState::Options, &mut game_assets, &mut game_state);
        }
        if *selected_button == 1 {
            exit.send(AppExit);
        }
    }
}

fn handle_controllers(
    controllers: Res<game_controller::GameController>,
    mut players: Query<(Entity, &mut ActionState<MenuAction>)>,
) {
    for (_, mut action_state) in players.iter_mut() {
        for (_, just_pressed) in controllers.just_pressed.iter() {
            // release all buttons
            // this probably affects durations but for
            // this game it might not be a big deal
            action_state.release(MenuAction::Up);
            action_state.release(MenuAction::Down);

            action_state.release(MenuAction::Select);

            if just_pressed.contains(&game_controller::GameButton::Up) {
                action_state.press(MenuAction::Up);
            }
            if just_pressed.contains(&game_controller::GameButton::Down) {
                action_state.press(MenuAction::Down);
            }
            if just_pressed.contains(&game_controller::GameButton::ActionDown)
                || just_pressed.contains(&game_controller::GameButton::Start)
            {
                action_state.press(MenuAction::Select);
            }
        }
    }
}

