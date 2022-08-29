use crate::{
    assets::GameAssets, cleanup, game_state, menus, AppState, ui::text_size, ingame, other_persons,
    component_adder::AnimationLink, game_camera, ingame_ui, title_screen::MenuAction, LEFT_GOAL, football,
    asset_loading, audio::GameAudio, 
};
use std::mem;
use bevy::prelude::*;
use rand::Rng;
use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::InputManagerBundle;

pub struct CutscenePlugin;
impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Cutscene)
           .with_system(play_cutscene)
           .with_system(display_textbox)
           .with_system(handle_input)
           .with_system(move_camera)
           .with_system(game_camera::pan_orbit_camera)

        )
        .add_system_set(SystemSet::on_update(AppState::InGame)
            .with_system(handle_cutscene_event)
        )
        .add_event::<CutsceneEvent>()
        .insert_resource(TextBox::default())
        .add_system_set(SystemSet::on_enter(AppState::Cutscene)
           .with_system(cleanup::<ingame_ui::CleanupMarker>)
           .with_system(setup_cutscene)
        )
        .add_system_set(SystemSet::on_exit(AppState::Cutscene)
           .with_system(cleanup::<CleanupMarker>)
           .with_system(cleanup::<ingame::CleanupMarker>)
        )
        .insert_resource(CutsceneState::default());
    }
}

#[derive(Component)]
struct CleanupMarker;

#[derive(Default)]
pub struct CutsceneState {
    pub current: Option::<Cutscene>,
    pub cutscene_index: usize,
    cooldown: f32,
    input_cooldown: f32,
    waiting_on_input: bool,
    target_camera_translation: Option::<Vec3>,
    target_camera_rotation: Option::<Quat>,
    camera_speed: f32,
    current_bill_animation: Handle<AnimationClip>,
    current_will_animation: Handle<AnimationClip>,
}

impl CutsceneState {
    pub fn init(&mut self, cutscene: Cutscene) {
        self.current = Some(cutscene);
        self.cutscene_index = 0;
        self.cooldown = 0.0;
        self.input_cooldown = 0.0;
        self.target_camera_translation = None;
        self.target_camera_rotation = None;
        self.waiting_on_input = false;
        self.current_bill_animation = Handle::<AnimationClip>::default();
        self.current_will_animation = Handle::<AnimationClip>::default();
    }
}

#[derive(Copy, Clone)]
pub enum Cutscene {
    Intro,
    Death,
    Tackle,
    LevelTwoIntro,
    LevelThreeIntro,
    RoundOneOver,
    RoundTwoOver,
    RoundThreeOver,
}

impl Default for Cutscene {
    fn default() -> Self {
        Cutscene::Intro
    }
}

#[derive(Component)]
pub struct CutsceneTextBoxContainer;
#[derive(Component)]
struct CutsceneTextContainerMarker;

fn play_cutscene(
    mut cutscene_state: ResMut<CutsceneState>,
    mut camera: Query<&mut Transform, With<game_camera::PanOrbitCamera>>,
    mut textbox: ResMut<TextBox>,
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<GameAssets>,
    mut game_state: ResMut<game_state::GameState>,
    mut will_animation_link: Query<&AnimationLink, With<other_persons::WillPerson>>,
    mut bill_animation_link: Query<&AnimationLink, With<other_persons::BillPerson>>,
    mut animations: Query<&mut AnimationPlayer>,
    mut football_launch_event_writer: EventWriter<football::LaunchFootballEvent>,
    mut ingame_ui_textbox: ResMut<ingame_ui::TextBox>,
    mut audio: GameAudio,
) {
    if let Ok(will_link_check) = will_animation_link.get_single() {
        if will_link_check.entity.is_none() {
            return;
        }
    } else {
        return;
    }
    if let Ok(bill_link_check) = bill_animation_link.get_single() {
        if bill_link_check.entity.is_none() {
            return;
        }
    } else {
        return;
    }

    let mut camera = camera.single_mut();
//    println!("{:?} {:?}", camera.translation, camera.rotation.to_axis_angle());
    if cutscene_state.waiting_on_input { return; }
    let mut bill_animation = None;
    let mut will_animation = None;

    cutscene_state.camera_speed = 2.0;
    cutscene_state.waiting_on_input = true;
    let text_speed = 0.10;

    if let Some(current) = cutscene_state.current {
        *ingame_ui_textbox = ingame_ui::TextBox::default(); // clear out any banter or commentary
        match current {
            Cutscene::LevelTwoIntro => {
                match cutscene_state.cutscene_index {
                    0 => {
                        camera.translation = Vec3::new(22.5, 1.5, 0.0);
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                        audio.stop_bgm();
                        cutscene_state.target_camera_translation = Some((Vec3::new(19.3, 1.5, 0.0)));
                        textbox.queued_text = Some(TextBoxText {
                            text: "Welcome back!".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_right.clone()); 
                    },
                    1 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "The field is ready for the next round and so are we!".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_right.clone()); 
                    },
                    2 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "FOUR HOURS".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    3 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Yeah, we ran into some difficulties.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_look_left_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    4 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "But, we're ready now. We have popcorn. Let's get started!".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    _ => {
                        camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                                       game_camera::INGAME_CAMERA_Y, 
                                                       LEFT_GOAL);
                        camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                                    game_camera::INGAME_CAMERA_ROTATION_ANGLE);
                        audio.play_bgm(&game_assets.bgm);
                        game_state.corn_spawned = true;
                        cutscene_state.current = None;
                        assets_handler.load(AppState::ResetInGame, &mut game_assets, &game_state);
                    }
                }
            },
            Cutscene::LevelThreeIntro => {
                match cutscene_state.cutscene_index {
                    0 => {
                        camera.translation = Vec3::new(22.5, 1.5, 0.0);
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                        audio.stop_bgm();
                        cutscene_state.target_camera_translation = Some((Vec3::new(19.3, 1.5, 0.0)));
                        textbox.queued_text = Some(TextBoxText {
                            text: "And we're back!".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    1 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "As you can tell, it's quite late.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    2 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "We are keeping warm while eating our elotes.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    3 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "I'm having esquites!".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    4 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "That's the same thing".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    5 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "it really isn't.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    6 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Anyway!".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    7 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "We've been cited by the city for the stadium lights being lit after midnight.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_right.clone()); 
                    },
                    8 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "We've made an agreement that if we're quiet, we can use two flood lights.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    9 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Players will have to navigate the maze in the dark.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    10 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "They'll be illuminated only by the lights tracking them.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    11 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Dealing with these last-minute adjustments is part of the challenge!".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    12 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Will, what do you think?".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    13 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "I can't see anything.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_talk.clone()); 
                    },
                    14 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Fantastic! Let's begin!".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    _ => {
                        camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                                       game_camera::INGAME_CAMERA_Y, 
                                                       LEFT_GOAL);
                        camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                                    game_camera::INGAME_CAMERA_ROTATION_ANGLE);
                        audio.play_bgm(&game_assets.bgm);
                        game_state.corn_spawned = true;
                        cutscene_state.current = None;
                        assets_handler.load(AppState::ResetInGame, &mut game_assets, &game_state);
                    }
                }
            },
            Cutscene::Intro => {
                match cutscene_state.cutscene_index {
                    0 => {
                        camera.translation = Vec3::new(22.5, 1.5, 0.0);
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                        cutscene_state.target_camera_translation = Some((Vec3::new(19.3, 1.5, 0.0)));
                        textbox.queued_text = Some(TextBoxText {
                            text: "Hello! I'm Bill.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    1 => {
                        cutscene_state.target_camera_translation = None;
                        textbox.queued_text = Some(TextBoxText {
                            text: "and I'm Will!".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_idle.clone()); 
                        will_animation = Some(game_assets.host_talk.clone());
                    },
                    2 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "and we're here live from the USAFL Scouting Combine XLV in Indianapolis!".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    3 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "it's very exciting!".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    4 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "I've been looking forward to this event all year.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    5 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "ha ha ha, ..yeah.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    6 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "well, if you're just joining in at home and have no idea what's going on..".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_talk.clone()); 
                    },
                    7 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "The USAFL Scouting Combine is an annual, week-long showcase where athletes perform mental and physical trials to potentially be drafted on an USAFL team.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_talk.clone()); 
                    },
                    8 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "We're already a couple days in and most of the crowd-favorite events have passed, but everyone's pumped for this year's new challenge.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_idle.clone()); 
                        will_animation = Some(game_assets.host_talk.clone()); 
                    },
                    9 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "The Combine Combine Challenge!".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_idle.clone()); 
                        will_animation = Some(game_assets.host_talk.clone()); 
                    },
                    10 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "It's very exciting.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    11 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "very exciting".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    12 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Bill, I lost my notes, can you tell our viewers what it's all about?".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    13 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Ha ha, that keeps happening why is that? ha ha".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    14 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "The challenge is for players to score as many touchdowns as they can while navigating a corn maze".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    15 => {
                        cutscene_state.target_camera_translation = Some((Vec3::new(-18.6, 6.5, 16.041729)));
                        cutscene_state.target_camera_rotation = Some(
                            Quat::from_axis_angle(Vec3::new(-0.5818577, -0.7968926, -0.1624936), 0.6599797));
                        textbox.queued_text = Some(TextBoxText {
                            text: "As you can see the field is set.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    16 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "We've been generously donated 1,000 acres worth of corn.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    17 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "And between attempts we have a team of 200 volunteers meticulously re-constructing the mazes.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    18 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "This ensures each player has the same exact maze so there aren't any unfair advantages.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    19 => {
                        camera.translation = Vec3::new(19.3, 1.5, 0.0);
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);

                        cutscene_state.target_camera_translation = None;
                        cutscene_state.target_camera_rotation = None;

                        textbox.queued_text = Some(TextBoxText {
                            text: "That seems excessive.. that's like a lot of corn.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    20 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Ha ha.. yeah".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    21 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Let's talk more about how this works".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_right.clone()); 
                    },
                    22 => {
                        cutscene_state.target_camera_translation = Some(Vec3::new(-13.1, 2.5, -40.6));
                        cutscene_state.target_camera_rotation = Some(
                            Quat::from_axis_angle(Vec3::new(-0.07643463, -0.9914023, -0.10620499), 1.8807149));

                        textbox.queued_text = Some(TextBoxText {
                            text: "Players will start on one side of the field and a kicker will launch a ball into the maze.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_right.clone()); 
                    },
                    23 => {
                        cutscene_state.target_camera_translation = Some(Vec3::new(-11.3, 7.9, 14.1));
                        cutscene_state.target_camera_rotation = Some(
                            Quat::from_axis_angle(Vec3::new(-0.50110954, -0.84660023, -0.17932819), 0.78708464));

                        cutscene_state.camera_speed = 0.2;
                        textbox.queued_text = Some(TextBoxText {
                            text: "The player will navigate the maze, find the ball and score a point on the other side.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_right.clone()); 
                    },
                    24 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Once they score, a ball will be launched from the opposite side into the maze.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_right.clone()); 
                    },
                    25 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "And the player will have to turn back, find the ball and score a touchdown on the opposite side.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_right.clone()); 
                    },
                    26 => {
                        camera.translation = Vec3::new(19.3, 1.5, 0.0);
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                        cutscene_state.camera_speed = 2.0;
                        cutscene_state.target_camera_translation = None;
                        cutscene_state.target_camera_rotation = None;
                        textbox.queued_text = Some(TextBoxText {
                            text: "And the cycle just repeats from there.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    27 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "That's a lot of running.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    28 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Yeah..".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    29 => {
                        camera.translation = Vec3::new(-11.3, 7.9, 14.1);
                        cutscene_state.target_camera_translation = Some(Vec3::new(-11.3, 7.9, -14.1));
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.50110954, -0.84660023, -0.17932819), 0.78708464);
                        cutscene_state.camera_speed = 0.2;
                        textbox.queued_text = Some(TextBoxText {
                            text: "In the maze, we have seasoned USAFL players ready to chase and tackle the player.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    30 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Players won't be able to run as fast if they have one of these professionals holding them down.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    31 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "And if they get tackled, they'll have to go back to the goal line.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    32 => {
                        camera.translation = Vec3::new(19.3, 1.5, 0.0);
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                        cutscene_state.target_camera_translation = None;
                        cutscene_state.target_camera_rotation = None;
                        textbox.queued_text = Some(TextBoxText {
                            text: "I think that about covers everything.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    33 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "What about the combine?".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    34 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Oh! Right! The combine!".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    35 => {
                        cutscene_state.target_camera_translation = Some(Vec3::new(-3.0, 6.5, 28.7));
                        cutscene_state.target_camera_rotation = Some(
                            Quat::from_axis_angle(Vec3::new(-0.09976758, -0.9702991, -0.22037746), 2.3035543));

                        textbox.queued_text = Some(TextBoxText {
                            text: "While the player is attempting to score, a combine will be harvesting the maze.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    36 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Don't worry though, we have a professional driver in the combine.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    37 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Players are equipped with special padding that can't be sliced by the combine's blades. They should just safely bounce away.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    38 => {
                        camera.translation = Vec3::new(19.3, 1.5, 0.0);
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                        cutscene_state.target_camera_translation = None;
                        cutscene_state.target_camera_rotation = None;
                        textbox.queued_text = Some(TextBoxText {
                            text: "Bill, this seems really dangerous.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    39 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "We have an ambulance on site. And the players have signed waivers.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    40 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "...".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::None,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    41 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "The round ends once the combine has completely harvested the maze.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_right.clone()); 
                    },
                    42 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Then, we'll have our team re-plant the maze for the next round.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    43 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "They just... plant that right into the astroturf?".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    44 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Yeah.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    45 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "We know we have players with different preferences on how to compete.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    46 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Players can use the WASD keys, the Arrow keys or ZQSD keys to navigate the maze.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    47 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "...".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::None,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    48 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Bill.. what are you talking about?".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    49 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Ha ha, Will, football is played a little different these days.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    50 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "I think that about covers everything though, right?".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    51 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Actually, this always bugged me.. why is it called \"Combine\"?".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    52 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "OH, that's just because there used to be several scouting showcases across the country.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    53 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "and then one year they decided to combine them into one.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    54 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "...and so instead of calling it the \"Scouting Showcase\", they called it.. \"Combine\"?".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    55 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "ha... yeah.. I uhh.. that's what it's called, man, I don't know.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    56 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Oh, the kicker is ready, I think it's about to begin. Let's watch!".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    _ => {
                        camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                                       game_camera::INGAME_CAMERA_Y, 
                                                       LEFT_GOAL);
                        camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                                    game_camera::INGAME_CAMERA_ROTATION_ANGLE);
                        game_state.corn_spawned = true;
                        cutscene_state.current = None;
                        assets_handler.load(AppState::ResetInGame, &mut game_assets, &game_state);
                        audio.play_bgm(&game_assets.bgm);
                    }
                }
            },
            Cutscene::Tackle => {
                match cutscene_state.cutscene_index {
                    0 => {
                        camera.translation = Vec3::new(19.3, 1.5, 0.0);
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                        cutscene_state.target_camera_translation = None;
                        cutscene_state.target_camera_rotation = None;
                        textbox.queued_text = Some(TextBoxText {
                            text: "Oooph, that's a tackle! Back to the goal line.".to_string(),
                            speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    _ => {
                        camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                                       game_camera::INGAME_CAMERA_Y, 
                                                       LEFT_GOAL);
                        camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                                    game_camera::INGAME_CAMERA_ROTATION_ANGLE);
                        cutscene_state.current = None;
                        assets_handler.load(AppState::ResetInGame, &mut game_assets, &game_state);
                    }
                }
            },
            Cutscene::RoundOneOver => {
                match cutscene_state.cutscene_index {
                    0 => {
                        camera.translation = Vec3::new(19.3, 1.5, 0.0);
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                        audio.stop_bgm();
                        cutscene_state.target_camera_translation = None;
                        cutscene_state.target_camera_rotation = None;
                        textbox.queued_text = Some(TextBoxText {
                            text: "Well! That's it for round one!".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    1 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "That was a great performance.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });
                        bill_animation = Some(game_assets.host_idle.clone()); 
                        will_animation = Some(game_assets.host_talk.clone()); 
                    },
                    2 => {
                        if game_state.score == 0 {
                            textbox.queued_text = Some(TextBoxText {
                                text: "Yeah.. no touch downs though..".to_string(),
                                speed: text_speed,
                                auto: false,
                                speaking: DisplayCharacter::Bill,
                            });
                        } else if game_state.score == 100 {
                            textbox.queued_text = Some(TextBoxText {
                                text: "It was only one, but it was a great touchdown.".to_string(),
                                speed: text_speed,
                                auto: false,
                                speaking: DisplayCharacter::Bill,
                            });
                        } else {
                            textbox.queued_text = Some(TextBoxText {
                                text: "Always great to see a couple touchdowns.".to_string(),
                                speed: text_speed,
                                auto: false,
                                speaking: DisplayCharacter::Bill,
                            });
                        }
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    3 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "The volunteers are taking to the field now to re-plant.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    4 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Hey uh... how long until the field is ready?".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    5 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "... should only take a couple hours tops.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    6 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "WHAT".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    _ => {
                        camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                                       game_camera::INGAME_CAMERA_Y, 
                                                       LEFT_GOAL);
                        camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                                    game_camera::INGAME_CAMERA_ROTATION_ANGLE);
                        cutscene_state.current = None;
                        assets_handler.load(AppState::LevelOver, &mut game_assets, &game_state);
                    }
                }
            },
            Cutscene::RoundTwoOver => {
                match cutscene_state.cutscene_index {
                    0 => {
                        camera.translation = Vec3::new(19.3, 1.5, 0.0);
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                        audio.stop_bgm();
                        cutscene_state.target_camera_translation = None;
                        cutscene_state.target_camera_rotation = None;
                        textbox.queued_text = Some(TextBoxText {
                            text: "Wooooo! This is how football was always meant to be!".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    1 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Yeah, I have to admit, that was very exciting.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    2 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Very exciting.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    3 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Well uhh.. I guess we have to replant now..".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    4 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Yeah...".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    5 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "We'll be back after the break!".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    _ => {
                        camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                                       game_camera::INGAME_CAMERA_Y, 
                                                       LEFT_GOAL);
                        camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                                    game_camera::INGAME_CAMERA_ROTATION_ANGLE);
                        cutscene_state.current = None;
                        assets_handler.load(AppState::LevelOver, &mut game_assets, &game_state);
                    }
                }
            },
            Cutscene::RoundThreeOver => {
                match cutscene_state.cutscene_index {
                    0 => {
                        camera.translation = Vec3::new(19.3, 1.5, 0.0);
                        camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                        audio.stop_bgm();
                        cutscene_state.target_camera_translation = None;
                        cutscene_state.target_camera_rotation = None;
                        textbox.queued_text = Some(TextBoxText {
                            text: "What time is it?".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });
                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    1 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "It's 3:37am".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    2 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Are these taking longer? We got through three rounds today.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    3 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Yeah, we had to send all the other players home early.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    4 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "The scouts didn't even stay for this last round.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    5 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "I.. I don't think this is the best event for the Combine.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    6 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "No.. it didn't go quite as planned.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    7 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "We did get a lot of corn out of it though.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    8 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "...".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::None,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    9 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "I like corn, Will.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    10 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "I know.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left_talk.clone()); 
                    },
                    11 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "...".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::None,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_look_left.clone()); 
                    },
                    12 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Anyway! If you're still out there, thanks for sticking around.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    13 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "We're going to have some internal meetings to tweak this event.".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    14 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "Join us again next time for more USA Football League Scouting Combine coverage!".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Bill,
                        });

                        bill_animation = Some(game_assets.host_talk.clone()); 
                        will_animation = Some(game_assets.host_idle.clone()); 
                    },
                    15 => {
                        textbox.queued_text = Some(TextBoxText {
                            text: "What a mess".to_string(),
                            speed: text_speed,
                            auto: false,
                            speaking: DisplayCharacter::Will,
                        });

                        bill_animation = Some(game_assets.host_look_right.clone()); 
                        will_animation = Some(game_assets.host_talk.clone()); 
                    },
                    _ => {
                        cutscene_state.current = None;
                        *game_state = game_state::GameState::default();
                        assets_handler.load(AppState::TitleScreen, &mut game_assets, &game_state);
                    }
                }
            },
            Cutscene::Death => {
                match game_state.death_count {
                    1 => {
                        match cutscene_state.cutscene_index {
                            0 => {
                                camera.translation = Vec3::new(19.3, 1.5, 0.0);
                                camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                                audio.stop_bgm();
                                cutscene_state.target_camera_translation = None;
                                cutscene_state.target_camera_rotation = None;
                                textbox.queued_text = Some(TextBoxText {
                                    text: "Oh geez! The padding failed! What happened!?".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                                });
                                bill_animation = Some(game_assets.host_talk.clone()); 
                                will_animation = Some(game_assets.host_idle.clone()); 
                            },
                            1 => {
                                textbox.queued_text = Some(TextBoxText {
                                    text: "Someone call the medics! ".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                                });
                                bill_animation = Some(game_assets.host_idle.clone()); 
                                will_animation = Some(game_assets.host_talk.clone()); 
                            },
                            2 => {
                                textbox.queued_text = Some(TextBoxText {
                                    text: "I think... I think they're getting out...".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                                });
                                bill_animation = Some(game_assets.host_talk.clone()); 
                                will_animation = Some(game_assets.host_idle.clone()); 
                            },
                            3 => {
                                textbox.queued_text = Some(TextBoxText {
                                    text: "...".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::None,
                                });
                                bill_animation = Some(game_assets.host_idle.clone()); 
                                will_animation = Some(game_assets.host_look_left.clone()); 
                            },
                            4 => {
                                textbox.queued_text = Some(TextBoxText {
                                    text: "it.. it looks like they're ok?".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                                });
                                bill_animation = Some(game_assets.host_talk.clone()); 
                                will_animation = Some(game_assets.host_idle.clone()); 
                            },
                            5 => {
                                textbox.queued_text = Some(TextBoxText {
                                    text: "They're standing up.. they just gave a thumbs-up.".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                                });
                                bill_animation = Some(game_assets.host_talk.clone()); 
                                will_animation = Some(game_assets.host_idle.clone()); 
                            },
                            6 => {
                                textbox.queued_text = Some(TextBoxText {
                                    text: "I guess.. I guess we can just continue from here?".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                                });
                                bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                                will_animation = Some(game_assets.host_look_left.clone()); 
                            },
                            _ => {
                                camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                                               game_camera::INGAME_CAMERA_Y, 
                                                               LEFT_GOAL);
                                camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                                            game_camera::INGAME_CAMERA_ROTATION_ANGLE);
                                audio.play_bgm(&game_assets.bgm);
                                cutscene_state.current = None;
                                assets_handler.load(AppState::ResetInGame, &mut game_assets, &game_state);
                            }
                        }
                    },
                    2 => {
                        match cutscene_state.cutscene_index {
                            0 => {
                                camera.translation = Vec3::new(19.3, 1.5, 0.0);
                                camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                                audio.stop_bgm();
                                cutscene_state.target_camera_translation = None;
                                cutscene_state.target_camera_rotation = None;
                                textbox.queued_text = Some(TextBoxText {
                                    text: "OH NO NOT AGAIN!".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                                });
                                bill_animation = Some(game_assets.host_talk.clone()); 
                                will_animation = Some(game_assets.host_idle.clone()); 
                            },
                            1 => {
                                textbox.queued_text = Some(TextBoxText {
                                    text: "You said the driver was a professional!".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                                });
                                bill_animation = Some(game_assets.host_idle.clone()); 
                                will_animation = Some(game_assets.host_talk.clone()); 
                            },
                            2 => {
                                textbox.queued_text = Some(TextBoxText {
                                    text: "...".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::None,
                                });
                                bill_animation = Some(game_assets.host_idle.clone()); 
                                will_animation = Some(game_assets.host_idle.clone()); 
                            },
                            3 => {
                                textbox.queued_text = Some(TextBoxText {
                                    text: "Wow, they're ok. I can't believe it.".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                                });
                                bill_animation = Some(game_assets.host_idle.clone()); 
                                will_animation = Some(game_assets.host_look_left_talk.clone()); 
                            },
                            4 => {
                                textbox.queued_text = Some(TextBoxText {
                                    text: "That was worst than the first time.".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                                });
                                bill_animation = Some(game_assets.host_look_right.clone()); 
                                will_animation = Some(game_assets.host_look_left_talk.clone()); 
                            },
                            5 => {
                                textbox.queued_text = Some(TextBoxText {
                                    text: "Well.. ok, let's keep going.".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Bill,
                                });
                                bill_animation = Some(game_assets.host_look_right_talk.clone()); 
                                will_animation = Some(game_assets.host_look_left.clone()); 
                            },
                            _ => {
                                camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                                               game_camera::INGAME_CAMERA_Y, 
                                                               LEFT_GOAL);
                                camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                                            game_camera::INGAME_CAMERA_ROTATION_ANGLE);
                                audio.play_bgm(&game_assets.bgm);
                                cutscene_state.current = None;
                                assets_handler.load(AppState::ResetInGame, &mut game_assets, &game_state);
                            }
                        }
                    },
                    3 => {
                        match cutscene_state.cutscene_index {
                            0 => {
                                camera.translation = Vec3::new(19.3, 1.5, 0.0);
                                camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                                cutscene_state.target_camera_translation = None;
                                cutscene_state.target_camera_rotation = None;
                                textbox.queued_text = Some(TextBoxText {
                                    text: "...Are they.. are they trying to do this?".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::Will,
                                });
                                bill_animation = Some(game_assets.host_look_right.clone()); 
                                will_animation = Some(game_assets.host_look_left_talk.clone()); 
                            },
                            _ => {
                                camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                                               game_camera::INGAME_CAMERA_Y, 
                                                               LEFT_GOAL);
                                camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                                            game_camera::INGAME_CAMERA_ROTATION_ANGLE);
                                cutscene_state.current = None;
                                assets_handler.load(AppState::ResetInGame, &mut game_assets, &game_state);
                            }
                        }
                    }
                    _ => {
                        match cutscene_state.cutscene_index {
                            0 => {
                                camera.translation = Vec3::new(19.3, 1.5, 0.0);
                                camera.rotation = Quat::from_axis_angle(Vec3::new(-0.034182332, -0.9987495, -0.03648749), 1.5735247);
                                cutscene_state.target_camera_translation = None;
                                cutscene_state.target_camera_rotation = None;
                                textbox.queued_text = Some(TextBoxText {
                                    text: "...".to_string(),
                                    speed: text_speed,
                                    auto: false,
                                    speaking: DisplayCharacter::None,
                                });
                                bill_animation = Some(game_assets.host_look_right.clone()); 
                                will_animation = Some(game_assets.host_look_left.clone()); 
                            },
                            _ => {
                                camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                                               game_camera::INGAME_CAMERA_Y, 
                                                               LEFT_GOAL);
                                camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                                            game_camera::INGAME_CAMERA_ROTATION_ANGLE);
                                cutscene_state.current = None;
                                assets_handler.load(AppState::ResetInGame, &mut game_assets, &game_state);
                            }
                        }
                    }
                }
            }
            _ => ()
        }
    } else {
        camera.translation = Vec3::new(game_camera::INGAME_CAMERA_X, 
                                       game_camera::INGAME_CAMERA_Y, 
                                       LEFT_GOAL);
        camera.rotation = Quat::from_axis_angle(game_camera::INGAME_CAMERA_ROTATION_AXIS, 
                                    game_camera::INGAME_CAMERA_ROTATION_ANGLE);
        cutscene_state.current = None;
        assets_handler.load(AppState::ResetInGame, &mut game_assets, &game_state);
    }

    if let Some(will_animation) = will_animation {
        for will_animation_link in &will_animation_link {
            if let Some(animation_entity) = will_animation_link.entity {
                let mut animation = animations.get_mut(animation_entity).unwrap();
                if cutscene_state.current_will_animation != will_animation {
                    animation.play(will_animation.clone_weak()).repeat();
                    animation.resume();
                    cutscene_state.current_will_animation = will_animation.clone_weak();
                    animation.set_speed(4.0);
                } 
            }
        }
    }
    if let Some(bill_animation) = bill_animation {
        for bill_animation_link in &bill_animation_link {
            if let Some(animation_entity) = bill_animation_link.entity {
                let mut animation = animations.get_mut(animation_entity).unwrap();
                if cutscene_state.current_bill_animation != bill_animation {
                    animation.play(bill_animation.clone_weak()).repeat();
                    animation.resume();
                    cutscene_state.current_bill_animation = bill_animation.clone_weak();
                    animation.set_speed(4.0);
                } 
            }
        }
    }
}

fn move_camera(
    mut cutscene_state: ResMut<CutsceneState>,
    mut camera: Query<&mut Transform, With<game_camera::PanOrbitCamera>>,
    time: Res<Time>
) {
    if let Some(target) = cutscene_state.target_camera_translation {
        let mut camera = camera.single_mut();
        let camera_translation = camera.translation;
        camera.translation += (target - camera_translation) * (time.delta_seconds() * cutscene_state.camera_speed.max(0.1));
    }
    if let Some(target) = cutscene_state.target_camera_rotation {
        let mut camera = camera.single_mut();
        let new_rotation = camera.rotation
                            .lerp(target, time.delta_seconds() * cutscene_state.camera_speed.max(0.1));
        if !new_rotation.is_nan() {
            camera.rotation = new_rotation;
        }
    }
}

fn setup_cutscene(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut game_state: ResMut<game_state::GameState>,
    text_scaler: text_size::TextScaler,
) {
    commands
        .spawn_bundle(InputManagerBundle {
            input_map: MenuAction::default_input_map(),
            action_state: ActionState::default(),
        })
        .insert(CleanupMarker);

    let scale = (text_scaler.window_size.width * 0.1) / ingame::RENDER_TEXTURE_SIZE as f32;

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..Default::default()
                })
                .insert(CutsceneTextBoxContainer)
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
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .insert(CutsceneTextContainerMarker);
                });
        });
}

#[derive(Copy, Clone)]
pub struct CutsceneEvent {
    pub cutscene: Cutscene
}
fn handle_cutscene_event(
    mut cutscene_event_reader: EventReader<CutsceneEvent>,
    mut app_state: ResMut<State<AppState>>,
    mut cutscene_state: ResMut<CutsceneState>,
) {
    for event in cutscene_event_reader.iter() {
        cutscene_state.init(event.cutscene);
    }

    if cutscene_state.current.is_some() {
        app_state.push(AppState::Cutscene).unwrap();
    }
}

pub struct TextBox {
    texts: Option::<Vec::<TextBoxText>>,
    queued_text: Option::<TextBoxText>,
    index: usize,
    cooldown: f32,
}

impl Default for TextBox {
    fn default() -> Self {
        TextBox {
            texts: None,
            queued_text: None,
            index: 0,
            cooldown: 0.0,
        }
    }
}

impl TextBox {
    fn take_next_text(&mut self) -> Option::<TextBoxText> {
        if let Some(texts) = &mut self.texts {
            if texts.is_empty() {
                None
            } else {
                Some(texts.remove(0))
            }
        } else {
            None
        }
    }
}

pub struct TextBoxText {
    text: String,
    speed: f32,
    auto: bool,
    speaking: DisplayCharacter,
}

enum DisplayCharacter {
    Bill, Will, None
}

fn queue_initial_text(
    mut textbox: ResMut<TextBox>,
) {
    textbox.queued_text = textbox.take_next_text();
    textbox.cooldown = textbox.queued_text.as_ref().unwrap().speed;
}

fn display_textbox(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut textbox: ResMut<TextBox>,
    mut text_container: Query<Entity, With<CutsceneTextContainerMarker>>,
    text_scaler: text_size::TextScaler,
    time: Res<Time>,
    mut audio: GameAudio,
) {
    textbox.cooldown -= time.delta_seconds();     
    textbox.cooldown = textbox.cooldown.clamp(-3.0, 3.0);
    if textbox.cooldown > 0.0 { return; }

    let mut current_speed = None;

    if let Ok(container) = text_container.get_single() {
        if let Some(current_text) = &mut textbox.queued_text {
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
                                    right: Val::Percent(1.0),
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
            match current_text.speaking {
                DisplayCharacter::Will => audio.play_talk(&game_assets.will_speak),
                DisplayCharacter::Bill => audio.play_talk(&game_assets.bill_speak),
                _ => ()
            }

            current_speed = Some(current_text.speed);
            if current_text.text.is_empty() {
                textbox.queued_text = None;
            }
        }
    }

    textbox.cooldown = current_speed.unwrap_or(textbox.cooldown);
}

fn handle_input(
    mut commands: Commands,
    action_state: Query<&ActionState<MenuAction>>,
    mut textbox: ResMut<TextBox>,
    text_container: Query<&Children, With<CutsceneTextContainerMarker>>,
    mut state: ResMut<State<AppState>>,
    mut cutscene_state: ResMut<CutsceneState>,
    time: Res<Time>,
) {
    if !cutscene_state.waiting_on_input { return; }

    cutscene_state.input_cooldown -= time.delta_seconds();     
    cutscene_state.input_cooldown = cutscene_state.input_cooldown.clamp(-3.0, 3.0);
    if cutscene_state.input_cooldown > 0.0 { return; }

    if let Ok(action_state) = action_state.get_single() {
        if action_state.just_pressed(MenuAction::Select) {
            cutscene_state.input_cooldown = 0.5;
            cutscene_state.waiting_on_input = false;
            cutscene_state.cutscene_index += 1;
            // clear out existing text
            for children in text_container.iter() {
                for entity in children.iter() {
                    commands.entity(*entity).despawn_recursive();
                }
            }
        }
    }
}
