use crate::{
    assets::GameAssets, cleanup, game_state, menus, AppState, ui::text_size, ingame, other_persons,
    component_adder::AnimationLink, game_camera, maze, audio::GameAudio,
};
use bevy::prelude::*;
use bevy::ui::UiColor;
use std::mem;
use bevy::render::{
    view::RenderLayers,
};

const WILL_POSITION: f32 = 1.0;
const BILL_POSITION: f32 = -1.0;
pub struct InGameUIPlugin;
impl Plugin for InGameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
                SystemSet::on_exit(AppState::ResetInGame)
                       .with_system(setup)
            )
            .insert_resource(TextBox::default())
            .add_event::<SetTextBoxEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_ui)
                    .with_system(display_textbox)
                    .with_system(handle_textbox_events)
                    //.with_system(detect_round_over),
            );
    }
}

#[derive(Component)]
pub struct CleanupMarker;

fn update_ui(
    game_state: Res<game_state::GameState>,
    mut score_indicators: Query<&mut Text, (With<ScoreIndicator>, Without<CornIndicator>)>,
    mut corn_indicators: Query<&mut Text, (With<CornIndicator>, Without<ScoreIndicator>)>,
    corn_stalks: Query<Entity, With<maze::CornStalk>>
) {
    for mut text in score_indicators.iter_mut() {
        text.sections[0].value = game_state.score.to_string();
    }
    for mut text in corn_indicators.iter_mut() {
        text.sections[0].value = corn_stalks.iter().len().to_string();
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut game_state: ResMut<game_state::GameState>,
    text_scaler: text_size::TextScaler,
) {
    let scale = (text_scaler.window_size.width * 0.1) / ingame::RENDER_TEXTURE_SIZE as f32;
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexEnd,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(ingame::CleanupMarker)
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(200.0), Val::Px(scale * ingame::RENDER_TEXTURE_SIZE as f32)),
                        margin: UiRect {
                            left: Val::Px(scale * ingame::RENDER_TEXTURE_SIZE as f32),
                            ..default()
                        },
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    visibility: Visibility {
                        is_visible: false
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..Default::default()
                })
                .insert(OuterTextBoxContainer)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexStart,
                                flex_wrap: FlexWrap::WrapReverse,
                                overflow: Overflow::Hidden,
                                ..Default::default()
                            },
                            color: Color::hex("2d3b95").unwrap().into(),
                            ..Default::default()
                        })
                        .insert(TextContainerMarker);
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexEnd,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                            position_type: PositionType::Relative,
                            justify_content: JustifyContent::FlexEnd,
                            align_items: AlignItems::FlexEnd,
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        add_title(
                            parent,
                            game_assets.font.clone(),
                            text_scaler.scale(menus::DEFAULT_FONT_SIZE * 0.8),
                            "0",
                            vec!(ScoreIndicator), // just an empty vec since can't do <impl Trait>
                        );
                        add_title(
                            parent,
                            game_assets.font.clone(),
                            text_scaler.scale(menus::DEFAULT_FONT_SIZE * 0.8),
                            "Pts ",
                            Vec::<ingame::CleanupMarker>::new(), // just an empty vec since can't do <impl Trait>
                        );
                    });
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                            position_type: PositionType::Relative,
                            justify_content: JustifyContent::FlexEnd,
                            align_items: AlignItems::FlexEnd,
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        add_title(
                            parent,
                            game_assets.font.clone(),
                            text_scaler.scale(menus::DEFAULT_FONT_SIZE * 0.8),
                            "0",
                            vec!(CornIndicator),
                        );
                        add_title(
                            parent,
                            game_assets.font.clone(),
                            text_scaler.scale(menus::DEFAULT_FONT_SIZE * 0.8),
                            "Corn",
                            Vec::<ingame::CleanupMarker>::new(), // just an empty vec since can't do <impl Trait>
                        );
                    });
                });

        });


//      let first_pass_layer = RenderLayers::layer(1);
      let scale = (text_scaler.window_size.width * 0.1) / ingame::RENDER_TEXTURE_SIZE as f32;
      let sprite_x = -(text_scaler.window_size.width / 2.0) + (ingame::RENDER_TEXTURE_SIZE as f32 / 2.0 * scale);
      let sprite_y = (text_scaler.window_size.height / 2.0) - (ingame::RENDER_TEXTURE_SIZE as f32 / 2.0 * scale);
      commands.spawn_bundle(SpriteBundle {
//                style: Style {
//                    size: Size::new(Val::Percent(30.0), Val::Auto),
//                    ..Default::default()
//                },
          transform: {
              let mut t = Transform::from_translation(Vec3::new(sprite_x, sprite_y, 0.0));
              t.apply_non_uniform_scale(Vec3::new(scale, scale, scale));
              t
          },
          visibility: Visibility {
              is_visible: false
          },
          texture: game_assets.bill_icon.image.clone(),
          ..Default::default()
        })
        .insert(HostSpriteMarker)
        .insert(OuterTextBoxContainer)
        .insert(ingame::CleanupMarker)
        .insert(CleanupMarker);
//        .insert(first_pass_layer) ;
}

#[derive(Component)]
pub struct HostSpriteMarker;
#[derive(Component)]
struct TextContainerMarker;
#[derive(Component)]
pub struct OuterTextBoxContainer;
#[derive(Component)]
pub struct TextBox {
    texts: Option::<Vec::<TextBoxText>>,
    queued_text: Option::<TextBoxText>,
    index: usize,
    cooldown: f32,
    current_animation: Handle<AnimationClip>,
    after_text_displayed_cooldown: f32,
}

impl Default for TextBox {
    fn default() -> Self {
        TextBox {
            texts: None,
            queued_text: None,
            index: 0,
            cooldown: 0.0,
            current_animation: Handle::<AnimationClip>::default(),
            after_text_displayed_cooldown: 0.0,
        }
    }
}

impl TextBox {
    fn take_next_text(&mut self) -> Option::<TextBoxText> {
        if let Some(texts) = &mut self.texts {
            if texts.is_empty() {
                None
            } else {
                let next_text = texts.remove(0);
                self.current_animation = Handle::<AnimationClip>::default();
                Some(next_text)
            }
        } else {
            None
        }
    }
}


#[derive(Clone)]
pub struct TextBoxText {
    pub text: String,
    pub speed: f32,
    pub character: DisplayCharacter,
    pub animation_clip: Handle<AnimationClip>,
    pub after_text_displayed_delay: f32,
}

#[derive(Clone)]
pub enum DisplayCharacter {
    Bill,
    Will
}

fn display_textbox(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut audio: GameAudio,
    mut textbox: ResMut<TextBox>,
    mut text_container: Query<(Entity, Option::<&Children>), With<TextContainerMarker>>,
    mut textbox_containers: Query<&mut Visibility, With<OuterTextBoxContainer>>,
    mut will_animation_link: Query<&AnimationLink, With<other_persons::WillPerson>>,
    mut bill_animation_link: Query<&AnimationLink, With<other_persons::BillPerson>>,
    mut animations: Query<&mut AnimationPlayer>,
    mut host_camera: Query<(&mut Transform, &game_camera::HostCamera)>,
    mut host_sprite: Query<&mut Handle<Image>, With<HostSpriteMarker>>,
    text_scaler: text_size::TextScaler,
    time: Res<Time>,
) {
    let still_displaying_last_text = textbox.after_text_displayed_cooldown > 0.0;
    textbox.after_text_displayed_cooldown -= time.delta_seconds();
    textbox.after_text_displayed_cooldown = textbox.after_text_displayed_cooldown.clamp(-3.0, 30.0);
    if textbox.after_text_displayed_cooldown > 0.0 { return; }

    if still_displaying_last_text {
        // clear out existing text
        textbox.cooldown = 0.0;
        for (_, children) in text_container.iter() {
            if let Some(children) = children {
                for entity in children.iter() {
                    commands.entity(*entity).despawn_recursive();
                }
            }
        }
        if !textbox.queued_text.is_some() {
            // stop displaying textbox
            *textbox = TextBox::default();
            for mut visibility in &mut textbox_containers {
                visibility.is_visible = false; 
            }
            return;
        }
    }

    textbox.cooldown -= time.delta_seconds();     
    textbox.cooldown = textbox.cooldown.clamp(-3.0, 3.0);
    if textbox.cooldown > 0.0 { return; }

    let mut current_speed = None;
    let mut text_done_data = None;
    let mut current_animation = textbox.current_animation.clone();

    if let Ok((container, _)) = text_container.get_single() {
        if let Some(current_text) = &mut textbox.queued_text {
            // setup the textbox
            for mut visibility in &mut textbox_containers {
                visibility.is_visible = true; 
            }

            match current_text.character {
                DisplayCharacter::Will => {
                    let mut host_sprite = host_sprite.single_mut();
                    *host_sprite = game_assets.will_icon.image.clone(); 

                    for (mut transform, _) in &mut host_camera {
                        transform.translation.z = WILL_POSITION;
                    }
                    for will_animation_link in &will_animation_link {
                        if let Some(animation_entity) = will_animation_link.entity {
                            let mut animation = animations.get_mut(animation_entity).unwrap();
                            if current_animation != current_text.animation_clip {
                                animation.play(current_text.animation_clip.clone_weak()).repeat();
                                animation.resume();
                                current_animation = current_text.animation_clip.clone_weak();
                                animation.set_speed(4.0);
                            } 
                        }
                    }
                    audio.play_talk(&game_assets.will_speak);
                },
                DisplayCharacter::Bill => {
                    let mut host_sprite = host_sprite.single_mut();
                    *host_sprite = game_assets.bill_icon.image.clone(); 

                    for (mut transform, _) in &mut host_camera {
                        transform.translation.z = BILL_POSITION;
                    }
                    for bill_animation_link in &bill_animation_link {
                        if let Some(animation_entity) = bill_animation_link.entity {
                            let mut animation = animations.get_mut(animation_entity).unwrap();
                            if current_animation != current_text.animation_clip {
                                animation.play(current_text.animation_clip.clone_weak()).repeat();
                                animation.resume();
                                current_animation = current_text.animation_clip.clone_weak();
                                animation.set_speed(4.0);
                            } 
                        }
                    }
                    audio.play_talk(&game_assets.bill_speak);
                }
            }

            // handle the text
            let maybe_space_index = current_text.text.find(' ');

            let text_to_display: String =
                if let Some(space_index) = maybe_space_index {
                    let mut temp = current_text.text.split_off(space_index + 1);
                    mem::swap(&mut temp, &mut current_text.text);
                    temp
                } else {
                    current_text.text.drain(..).collect()
                };

            let base_font_size = 50.0;
            let font_size = text_scaler.scale(base_font_size);
            commands.entity(container)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                margin: UiRect {
                                    right: Val::Px(text_scaler.scale(10.0)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text::from_section(
                                text_to_display.trim(),
                                TextStyle {
                                    font: game_assets.font.clone(),
                                    font_size,
                                    color: Color::WHITE,
                                }
                            ),
                            ..Default::default()
                        });
                    });

            current_speed = Some(current_text.speed);
            if current_text.text.is_empty() {
                text_done_data = Some(current_text.after_text_displayed_delay); 
            }
        }
    }

    textbox.current_animation = current_animation;

    if let Some(after_text_displayed_delay) = text_done_data {
        textbox.queued_text = textbox.take_next_text();
        textbox.after_text_displayed_cooldown = after_text_displayed_delay;
    }
    textbox.cooldown = current_speed.unwrap_or(textbox.cooldown);
}

pub struct SetTextBoxEvent {
    pub texts: Vec::<TextBoxText>,
}

fn handle_textbox_events(
    mut textbox_events: EventReader<SetTextBoxEvent>,
    mut textbox: ResMut<TextBox>,
) {
    for event in textbox_events.iter() {
        *textbox = TextBox::default();
        textbox.texts = Some(event.texts.to_vec()); 
        textbox.queued_text = textbox.take_next_text();
    }
}


#[derive(Component)]
struct ScoreIndicator;
#[derive(Component)]
struct CornIndicator;

pub fn add_title(
    builder: &mut ChildBuilder<'_, '_, '_>,
    font: Handle<Font>,
    font_size: f32,
    title: &str,
    mut components: Vec<impl Component>,
) {
    let mut text_bundle = builder.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            margin: UiRect {
//              left: Val::Percent(2.0),
//              right: Val::Auto,
                ..Default::default()
            },
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        text: Text::from_section(
            title.to_string(),
            TextStyle {
                font,
                font_size,
                color: Color::WHITE,
            },
        ).with_alignment(
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            }
        ),
        ..Default::default()
    });

    components.drain(..).for_each(|c| {
        text_bundle.insert(c);
    });
}



pub struct Commentary {
}
