use bevy::prelude::*;
use crate::{AppState, ui::text_size, assets::GameAssets, menus, cleanup, asset_loading, game_state};

pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Splash).with_system(setup))
            .init_resource::<SplashTracker>()
            .add_system_set(
                SystemSet::on_update(AppState::Splash).with_system(tick)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Splash).with_system(cleanup::<CleanupMarker>)
            );
    }
}

#[derive(Component)]
struct CleanupMarker;

#[derive(Default)]
struct SplashTracker {
    time: f32
}

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<GameAssets>,
) {
    assets_handler.add_material(&mut game_assets.bevy_icon, "textures/bevy.png", true);
    assets_handler.add_font(&mut game_assets.font, "fonts/monogram.ttf");
}

fn tick(
    time: Res<Time>,
    mut splash_tracker: ResMut<SplashTracker>,
    mut game_assets: ResMut<GameAssets>,
    mut game_state: ResMut<game_state::GameState>,
    mut assets_handler: asset_loading::AssetsHandler,
) {
    splash_tracker.time += time.delta_seconds();

    if splash_tracker.time > 3.0 {
        assets_handler.load(AppState::TitleScreen, &mut game_assets, &mut game_state);
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    text_scaler: text_size::TextScaler,
    mut splash_tracker: ResMut<SplashTracker>,
) {
    splash_tracker.time = 0.0;

    commands
        .spawn_bundle(Camera3dBundle {
            ..Default::default()
        })
        .insert(CleanupMarker);

   commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                flex_direction: FlexDirection::ColumnReverse,
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..Default::default()
                },
                ..Default::default()
            },
            color: UiColor(Color::rgba(1.00, 1.00, 1.00, 0.0)),
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Percent(60.0)),
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                image: game_assets.bevy_icon.image.clone().into(),
                ..Default::default()
            });

            parent.spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    "made with Bevy",
                    TextStyle {
                        font: game_assets.font.clone(),
                        font_size: text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.2),
                        color: Color::WHITE,
                    })
                    .with_alignment(
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        }),
                    ..Default::default()
                });
            });
}

