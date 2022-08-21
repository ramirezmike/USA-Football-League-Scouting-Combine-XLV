use bevy::prelude::*;
use crate::{
    AppState, maze::CornStalk, assets::GameAssets, component_adder::AnimationLink,
    collision::WorldAabb, game_state, ZeroSignum,
};
use bevy::render::primitives::Aabb;
use rand::thread_rng;
use rand::prelude::SliceRandom;

pub struct CombinePlugin;
impl Plugin for CombinePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(animate_combine)
                .with_system(seek_corn)
                .with_system(target_corn)
                .with_system(handle_corn_collision)
        );
    }
}

#[derive(Component)]
pub struct Combine {
    pub animation_set: bool,
    pub target_corn_stalk: Option::<Entity>,
    pub velocity: Vec3,
    pub speed: f32,
    pub rotation_speed: f32,
    pub friction: f32,
}

impl Default for Combine {
    fn default() -> Self {
        Combine {
            animation_set: false,
            target_corn_stalk: None,
            velocity: Vec3::default(),
            speed: 20.0,
            rotation_speed: 0.01,
            friction: 0.01,
        }
    }
}

#[derive(Component)]
pub struct CombineBlade;

const CORN_CUT_DISTANCE: f32 = 0.7;
fn handle_corn_collision( 
    mut corns: Query<(&mut CornStalk, &mut Transform), Without<Combine>>,
    combine_blades: Query<(&Transform, &CombineBlade, &Aabb, &GlobalTransform), Without<CornStalk>>,
) {
    for (blade_transform, blade, blade_aabb, blade_global_transform) in &combine_blades {
        let blade_global_matrix = blade_global_transform.compute_matrix();
        let blade_inverse_transform_matrix = blade_global_matrix.inverse();
        let min: Vec3 = blade_aabb.min().into();
        let max: Vec3 = blade_aabb.max().into();

        for (mut corn, mut corn_transform) in &mut corns {
            if corn.is_harvested { continue; }

            let corn_translation = corn_transform.translation;
            let corn_inverse = blade_inverse_transform_matrix.transform_point3(corn_translation);

            let corn_in_hitbox = corn_inverse.x > min.x
                              && corn_inverse.x < max.x
                              && corn_inverse.z > min.z
                              && corn_inverse.z < max.z;

            if corn_in_hitbox {
                corn_transform.scale.y = 0.1;
                corn.is_harvested = true;
            }
        }
    }
}

fn seek_corn(
    mut combines: Query<(&mut Combine, &mut Transform), Without<CornStalk>>,
    corns: Query<(&CornStalk, &Transform), Without<Combine>>,
    game_state: Res<game_state::GameState>,
    time: Res<Time>,
) {
    for (mut combine, mut combine_transform) in &mut combines {
        if combine.target_corn_stalk.is_none() { continue; }
        let corn_stalk_entity = combine.target_corn_stalk.expect("missing corn stalk");

        if let Ok((corn_stalk, corn_stalk_transform)) = corns.get(corn_stalk_entity) {
            if corn_stalk.is_harvested {
                combine.target_corn_stalk = None;
                continue;
            }

            let speed: f32 = combine.speed;
            let rotation_speed: f32 = combine.rotation_speed;
            let friction: f32 = combine.friction;

            combine.velocity *= friction.powf(time.delta_seconds());

            let direction = corn_stalk_transform.translation - 
                            (combine_transform.translation + (combine_transform.forward() * 1.1));
            let acceleration = Vec3::from(direction);
            combine.velocity += (acceleration.zero_signum() * speed) * time.delta_seconds();

            let new_translation = combine_transform.translation + (combine.velocity * time.delta_seconds());

            let angle = (-(new_translation.z - combine_transform.translation.z))
                .atan2(new_translation.x - combine_transform.translation.x);
            let rotation = Quat::from_axis_angle(Vec3::Y, angle);
            combine_transform.translation = new_translation;
            let new_rotation = combine_transform
                .rotation
                .lerp(rotation, time.delta_seconds() * rotation_speed);
            if !new_rotation.is_nan() && combine.velocity.length() > 0.001 {
                combine_transform.rotation = rotation;
            }
        }
    }
}

fn target_corn( 
    mut combines: Query<(&mut Combine, &Transform)>,
    corns: Query<(Entity, &CornStalk, &Transform)>,
) {
    for (mut combine, combine_transform) in &mut combines {
        if combine.target_corn_stalk.is_some() { continue; }

        let unharvested_corn = corns.iter()
                                    .filter(|(_, c, _)| !c.is_harvested)
                                    .collect::<Vec::<_>>();
        if unharvested_corn.is_empty() {
            println!("no more corn :(");
            // end of round?
            continue;
        }

        let mut rng = thread_rng();
        combine.target_corn_stalk = unharvested_corn.choose(&mut rng).map(|(e, _, _)| *e);
    }
}

fn animate_combine(
    mut combines: Query<(&mut Combine, &AnimationLink)>,
    mut animations: Query<&mut AnimationPlayer>,
    game_assets: ResMut<GameAssets>,
) {
    for (mut combine, animation_link) in &mut combines {
        if let Some(animation_entity) = animation_link.entity {
            if let Ok(mut animation) = animations.get_mut(animation_entity) {
                if !combine.animation_set {
                    animation.play(game_assets.combine_drive.clone_weak()).repeat();
                    combine.animation_set = true;
                }

                if animation.is_paused() {
                    animation.resume();
                }
            }
        }
    }
}
