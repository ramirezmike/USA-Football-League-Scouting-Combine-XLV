use crate::{
    assets::GameAssets, cleanup, game_state, menus, AppState, ui::text_size, ingame, other_persons,
    component_adder::AnimationLink, game_camera, ingame_ui,
};
use bevy::prelude::*;
use rand::Rng;

pub struct BanterPlugin;
impl Plugin for BanterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame)
               .with_system(send_banter)
        )
        .insert_resource(BanterState::default());
    }
}

#[derive(Default)]
pub struct BanterState {
    pub cooldown: f32,
    pub banters: Vec::<Banter>,
}

impl BanterState {
    pub fn reset(&mut self, game_assets: &GameAssets) {
        self.banters = generate_banter(game_assets);
        self.cooldown = BANTER_COOLDOWN;
    }
}

pub struct Banter {
    pub texts: Vec::<ingame_ui::TextBoxText>,
}

const BANTER_COOLDOWN: f32 = 10.0;

fn send_banter(
    mut banter_state: ResMut<BanterState>,
    time: Res<Time>,
    textbox_containers: Query<&Visibility, With<ingame_ui::OuterTextBoxContainer>>,
    mut textbox_event_writer: EventWriter<ingame_ui::SetTextBoxEvent>,
) {
    if banter_state.banters.is_empty() { return; }

    for visibility in &textbox_containers {
        if visibility.is_visible {
            banter_state.cooldown = BANTER_COOLDOWN;
            return;
        }
    }
    banter_state.cooldown -= time.delta_seconds();     
    banter_state.cooldown = banter_state.cooldown.clamp(-3.0, 30.0);

    if banter_state.cooldown > 0.0 { return; }

    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..10);
    if random > 5 {
        // keep waiting
        banter_state.cooldown = BANTER_COOLDOWN;
        return;
    }

    let random = rng.gen_range(0..banter_state.banters.len()) as usize;
    let selected_banter = banter_state.banters.swap_remove(random);
    println!("banters left: {}", banter_state.banters.len());

    textbox_event_writer.send(ingame_ui::SetTextBoxEvent {
        texts: selected_banter.texts.to_vec()
    });
    banter_state.cooldown = BANTER_COOLDOWN;
}

fn generate_banter(game_assets: &GameAssets) -> Vec::<Banter> {
    vec!(
        Banter {
            texts: vec!(
                bill_talk("I just realized our names rhyme", &game_assets),
                will_talk("Yeah...", &game_assets),
                bill_talk("Do you think that's why we're here?", &game_assets),
                will_silent("...", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("Lucky it's great weather today.", &game_assets),
                bill_talk("It can get pretty chilly in December.", &game_assets),
                will_talk("Bill... it's August.", &game_assets),
                bill_talk("Yeah, but what if it wasn't?", &game_assets),
                will_silent("...", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("yeah, and an order of chicken makhani.", &game_assets),
                will_talk("Bill!", &game_assets),
                bill_talk("and uhh, let's do level 7 spicy.", &game_assets),
                will_talk("Bill! Your mic is hot!", &game_assets),
                bill_talk("I'm not touching my mic..", &game_assets),
                bill_talk("OH!", &game_assets),
                will_silent("...", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("Why is the machine called combine?", &game_assets),
                will_talk("Oh, that's easy.", &game_assets),
                will_talk("The name derives from how it harvests.", &game_assets),
                will_talk("It combines four harvesting operations.", &game_assets),
                will_talk("Reaping, threshing, gathering..", &game_assets),
                will_talk("and winnowing.", &game_assets),
                bill_talk("o-oh ok.", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                will_talk("Are we going to have corn left over?", &game_assets),
                bill_talk("Yeah.", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("For me, I really like corn.", &game_assets),
                will_talk("What do you like about corn?", &game_assets),
                bill_talk("It's corn!", &game_assets),
                bill_talk("A big lump with knobs.", &game_assets),
                bill_talk("I can't imagine a more beautiful thing", &game_assets),
                will_silent("...", &game_assets),
                bill_talk("I can tell you all about it.", &game_assets),
                bill_talk("I mean, look at this thing.", &game_assets),
                bill_talk("When I tried it with butter..", &game_assets),
                bill_talk("EVERYTHING CHANGED", &game_assets),
                will_silent("...", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("Who does your mustache?", &game_assets),
                will_talk("d..does?", &game_assets),
                bill_talk("Yeah, like, where do you go?", &game_assets),
                will_talk("I trim my own mustache actually.", &game_assets),
                bill_talk("Like, with scissors?", &game_assets),
                will_talk("I have an electric razor.", &game_assets),
                bill_talk("that's cool.", &game_assets),
            )
        }
    )
}

fn will_talk(text: &str, game_assets: &GameAssets) -> ingame_ui::TextBoxText {
    ingame_ui::TextBoxText {
        text: text.to_string(),
        speed: 0.3,
        character: ingame_ui::DisplayCharacter::Will,
        animation_clip: game_assets.host_look_left_talk.clone(),
        after_text_displayed_delay: 1.0,
    }
}
fn will_silent(text: &str, game_assets: &GameAssets) -> ingame_ui::TextBoxText {
    ingame_ui::TextBoxText {
        text: text.to_string(),
        speed: 0.3,
        character: ingame_ui::DisplayCharacter::Will,
        animation_clip: game_assets.host_idle.clone(),
        after_text_displayed_delay: 1.0,
    }
}

fn bill_talk(text: &str, game_assets: &GameAssets) -> ingame_ui::TextBoxText {
    ingame_ui::TextBoxText {
        text: text.to_string(),
        speed: 0.3,
        character: ingame_ui::DisplayCharacter::Bill,
        animation_clip: game_assets.host_talk.clone(),
        after_text_displayed_delay: 1.0,
    }
}
fn bill_talk_r(text: &str, game_assets: &GameAssets) -> ingame_ui::TextBoxText {
    ingame_ui::TextBoxText {
        text: text.to_string(),
        speed: 0.3,
        character: ingame_ui::DisplayCharacter::Bill,
        animation_clip: game_assets.host_look_right_talk.clone(),
        after_text_displayed_delay: 1.0,
    }
}
