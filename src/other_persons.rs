use crate::{AppState, game_controller, direction, game_state, collision, assets::GameAssets, component_adder::AnimationLink, ZeroSignum, maze, player, LEFT_GOAL, RIGHT_GOAL, TOP_END, BOTTOM_END};
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use bevy::render::primitives::Aabb;
use std::f32::consts::{FRAC_PI_2, TAU};

pub struct OtherPersonsPlugin;
impl Plugin for OtherPersonsPlugin {
    fn build(&self, app: &mut App) {
//      app.add_system_set(
//          SystemSet::on_update(AppState::InGame)
//              .with_system(scale_lines_of_sight)
//              .with_system(handle_flying_enemies)
//              .with_system(handle_enemy_blade_event)
//              .with_system(move_enemy.after(scale_lines_of_sight)),
//      )
    }
}

#[derive(Component)]
pub struct BillPerson;
#[derive(Component)]
pub struct WillPerson;
