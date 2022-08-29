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
                bill_talk("Hey Will.", &game_assets),
                will_talk("What's up?", &game_assets),
                bill_talk("This event is.. pretty CORNY.", &game_assets),
                will_talk("Please don't do this.", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("THIS IS FOOOOOTBALLL!!!", &game_assets),
                will_talk("...", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("We should do this with other sports.", &game_assets),
                will_talk("Oh, like, other sports leagues?", &game_assets),
                bill_talk("No, this should involve more sports.", &game_assets),
                will_talk("...", &game_assets),
                bill_talk("Like the Olympics!", &game_assets),
                will_talk("I don't think players would like that.", &game_assets),
                bill_talk("It would be cool to watch though.", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                will_talk("Who thought of this?", &game_assets),
                bill_talk("I think an intern suggested it..", &game_assets),
                bill_talk("as a joke..", &game_assets),
                bill_talk("and someone wrote it down.", &game_assets),
                bill_talk("and then it got on the project board", &game_assets),
                bill_talk("as a joke..", &game_assets),
                bill_talk("and then we started calling farms..", &game_assets),
                bill_talk("and.. here we are.", &game_assets),
                will_talk("...", &game_assets),
                bill_talk("...yeah.", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("Oh, I almost forgot to mention", &game_assets),
                bill_talk("the sponsor for this event.", &game_assets),
                bill_talk("Special thanks to...", &game_assets),
                bill_talk("...Palatka Alpaca Apothecary.", &game_assets),
                will_talk("You said it right that time", &game_assets),
                bill_talk("I know, I've been practicing.", &game_assets),
                will_talk("Is that like.. a pharmacy?", &game_assets),
                bill_talk("I thought it was a boutique shop", &game_assets),
                will_talk("is it for alpacas?", &game_assets),
                bill_talk("I honestly have no idea.", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("Listen to any good music lately?", &game_assets),
                will_talk("I actually got really into mallsoft", &game_assets),
                bill_talk("mallsoft?", &game_assets),
                will_talk("Yeah, it's a vaporwave subgenre", &game_assets),
                will_talk("themed after retro shopping malls", &game_assets),
                will_talk("like listening to waking up in a mall", &game_assets),
                will_talk("soaked in nostalgia.", &game_assets),
                bill_talk("Oook...", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("Do you think you could do this?", &game_assets),
                will_talk("Absolutely not.", &game_assets),
                bill_talk("It's a lot of running.", &game_assets),
                bill_talk("but, I'm good at corn mazes...", &game_assets),
                will_talk("...", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                will_talk("You said the driver is professional?", &game_assets),
                bill_talk("yeah, harvesting for 25 years.", &game_assets),
                will_talk("but like...", &game_assets),
                will_talk("with people in the field?", &game_assets),
                bill_talk("Oh uh.. that's a good question", &game_assets),
                will_talk("...", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("Have you tried that new BBQ place?", &game_assets),
                will_talk("\"Smoke 'em if you got 'em\"?", &game_assets),
                bill_talk("Yeah.", &game_assets),
                bill_talk("They bring tiny smokers to your table", &game_assets),
                bill_talk("and you smoke your own meat.", &game_assets),
                bill_talk("it takes forever.", &game_assets),
                bill_talk("very expensive.", &game_assets),
                will_talk("...", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                will_talk("This reminds me of doing stadiums", &game_assets),
                bill_talk("Doing stadums?", &game_assets),
                will_talk("Yeah.", &game_assets),
                will_talk("You run up and down the stadium", &game_assets),
                will_talk("The whole way around", &game_assets),
                bill_talk("Why?", &game_assets),
                will_talk("It's fun!", &game_assets),
                bill_talk("...Is it?", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                bill_talk("Jim says hi by the way.", &game_assets),
                will_talk("Who?", &game_assets),
                bill_talk("Jim.", &game_assets),
                will_talk("I don't know a Jim.", &game_assets),
                bill_talk("Jim, from college.", &game_assets),
                will_talk("We didn't go to college together", &game_assets),
                bill_talk("Oh..", &game_assets),
                bill_talk("Well, he knows you.", &game_assets),
                will_silent("...", &game_assets),
            )
        },
        Banter {
            texts: vec!(
                will_talk("Oh Bill, how's your cat?", &game_assets),
                bill_talk("The vet says she's great!", &game_assets),
                bill_talk("They said she's the cutest cat.", &game_assets),
                will_talk("Yeah, she's a cute cat.", &game_assets),
                bill_talk("No, Will..", &game_assets),
                bill_talk("They said CUTEST.", &game_assets),
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
                bill_talk("We might keep the rest for next year", &game_assets),
                will_talk("...", &game_assets),
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
