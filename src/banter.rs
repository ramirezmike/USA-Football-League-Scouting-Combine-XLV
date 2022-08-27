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

const BANTER_COOLDOWN: f32 = 5.0;

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

    textbox_event_writer.send(ingame_ui::SetTextBoxEvent {
        texts: selected_banter.texts.to_vec()
    });
}

fn generate_banter(game_assets: &GameAssets) -> Vec::<Banter> {
    vec!(
        Banter {
            texts: vec!(
                bill_talk("ha ha ha", &game_assets),
            )
        }
    )
}

fn will_talk_l(text: &str, game_assets: &GameAssets) -> ingame_ui::TextBoxText {
    ingame_ui::TextBoxText {
        text: text.to_string(),
        speed: 1.01,
        character: ingame_ui::DisplayCharacter::Will,
        animation_clip: game_assets.host_look_left_talk.clone(),
        after_text_displayed_delay: 1.0,
    }
}
fn will_talk(text: &str, game_assets: &GameAssets) -> ingame_ui::TextBoxText {
    ingame_ui::TextBoxText {
        text: text.to_string(),
        speed: 1.01,
        character: ingame_ui::DisplayCharacter::Will,
        animation_clip: game_assets.host_talk.clone(),
        after_text_displayed_delay: 1.0,
    }
}

fn bill_talk(text: &str, game_assets: &GameAssets) -> ingame_ui::TextBoxText {
    ingame_ui::TextBoxText {
        text: text.to_string(),
        speed: 1.01,
        character: ingame_ui::DisplayCharacter::Bill,
        animation_clip: game_assets.host_talk.clone(),
        after_text_displayed_delay: 1.0,
    }
}
fn bill_talk_r(text: &str, game_assets: &GameAssets) -> ingame_ui::TextBoxText {
    ingame_ui::TextBoxText {
        text: text.to_string(),
        speed: 1.01,
        character: ingame_ui::DisplayCharacter::Bill,
        animation_clip: game_assets.host_look_right_talk.clone(),
        after_text_displayed_delay: 1.0,
    }
}

/*
   our names rhyme
   mustaches
   I LIKE CORN kid
   weather
   what are we going to do with all this corn
   why is the combine called a combine

*/
