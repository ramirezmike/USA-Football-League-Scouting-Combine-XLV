use crate::{AppState, game_controller, direction, game_state, collision, assets::GameAssets, component_adder::AnimationLink, ZeroSignum, maze, player};
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use bevy::render::primitives::Aabb;
use std::f32::consts::{FRAC_PI_2};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(scale_lines_of_sight)
                .with_system(move_enemy.after(scale_lines_of_sight)),
        );
    }
}

#[derive(Component)]
pub struct Enemy {
    pub line_of_sight: Entity,
    pub can_see_player: bool,
    pub velocity: Vec3,
    pub speed: f32,
    pub rotation_speed: f32,
    pub friction: f32,
    pub random: f32,
}

impl Enemy {
    pub fn new(line_of_sight: Entity) -> Self {
        let mut rng = rand::thread_rng();

        Enemy {
            line_of_sight,
            can_see_player: false,
            velocity: Vec3::default(),
            speed: 30.0,
            rotation_speed: 1.0,
            friction: 0.01,
            random: rng.gen_range(0.5..1.0),
        }
    }
}

#[derive(Component)]
pub struct EnemyLineOfSight;


fn scale_lines_of_sight(
    mut enemies: Query<(&mut Enemy, &Transform), Without<EnemyLineOfSight>>,
    mut lines_of_sight: Query<(&mut Transform, &Aabb, &GlobalTransform), With<EnemyLineOfSight>>,
    corns: Query<(&maze::CornStalk, &Transform), Without<EnemyLineOfSight>>,
    player: Query<&Transform, (Without<EnemyLineOfSight>, With<player::Player>)>,
) {
    let unharvested_corn = corns.iter()
                                .filter(|(c, _)| !c.is_harvested)
                                .collect::<Vec::<_>>();
    let LOS_LENGTH = 10.0;
    for (mut enemy, enemy_transform) in &mut enemies {
        if let Ok((mut line_of_sight, los_aabb, los_global_transform)) = lines_of_sight.get_mut(enemy.line_of_sight) {
            let los_global_matrix = los_global_transform.compute_matrix();
            let los_inverse_transform_matrix = los_global_matrix.inverse();
            let los_min: Vec3 = los_aabb.min().into();
            let los_max: Vec3 = los_aabb.max().into();

            let direction = enemy_transform.right().normalize();
            let end_of_sight = enemy_transform.translation + (LOS_LENGTH * direction);

            // check for player first
            let distance_to_player = {
                let player = player.single();
                let player_translation = player.translation;
                let player_inverse = los_inverse_transform_matrix.transform_point3(player_translation);
                let player_in_hitbox = player_inverse.x > los_min.x
                                    && player_inverse.x < los_max.x
                                    && player_inverse.z > los_min.z
                                    && player_inverse.z < los_max.z;
                if player_in_hitbox {
                    Some(player.translation.distance(enemy_transform.translation))
                } else {
                    None
                }
            };

            let mut corn_in_front_of_enemy = 
                unharvested_corn.iter()
                         .filter_map(|(c, t)| {
                             let corn_translation = t.translation;
                             let corn_inverse = los_inverse_transform_matrix.transform_point3(corn_translation);
                             let corn_in_hitbox = corn_inverse.x > los_min.x
                                               && corn_inverse.x < los_max.x
                                               && corn_inverse.z > los_min.z
                                               && corn_inverse.z < los_max.z;
                             if corn_in_hitbox {
                                 let distance = t.translation.distance(enemy_transform.translation);
                                 Some((c, t, distance))
                             } else {
                                 None
                             }
                         })
                         .collect::<Vec::<_>>();

            corn_in_front_of_enemy.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

            let end_of_sight = corn_in_front_of_enemy
                                .first()
                                .map(|(_, t, _)| t.translation)
                                .unwrap_or(end_of_sight);
            let middle = enemy_transform.translation.lerp(end_of_sight, 0.5);
            let line_of_sight_length = enemy_transform.translation.distance(end_of_sight);

            enemy.can_see_player = false;
            if let Some(distance_to_player) = distance_to_player {
                if line_of_sight_length > distance_to_player {
                    enemy.can_see_player = true;
                }
            } 

            line_of_sight.scale = Vec3::new(line_of_sight_length / 2.0, 1.00, 1.00);
            line_of_sight.translation = Vec3::new(middle.x, 1.0, middle.z);

            // Rotate the direction indicator
            if Vec3::Z.angle_between(direction) > FRAC_PI_2 {
                line_of_sight.rotation =
                    Quat::from_rotation_y(Vec3::X.angle_between(direction));
            } else {
                line_of_sight.rotation =
                    Quat::from_rotation_y(-Vec3::X.angle_between(direction));
            }
        }
    }
}

fn move_enemy(
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
    player: Query<&Transform, (With<player::Player>, Without<Enemy>)>,
    collidables: collision::Collidables,
    time: Res<Time>,
) {
    for (mut enemy, mut enemy_transform) in &mut enemies {
        let speed: f32 = enemy.speed;
        let rotation_speed: f32 = enemy.rotation_speed;
        let friction: f32 = enemy.friction;

        enemy.velocity *= friction.powf(time.delta_seconds());

        if enemy.can_see_player {
            let player = player.single();
            let direction = player.translation - enemy_transform.translation;
            let acceleration = Vec3::from(direction);

            enemy.velocity += (acceleration.zero_signum() * speed) * time.delta_seconds();
            enemy.velocity = enemy.velocity.clamp_length_max(speed);
        }

        let mut new_translation = enemy_transform.translation + (enemy.velocity * time.delta_seconds());
        collidables.fit_in(
            &enemy_transform.translation,
            &mut new_translation,
            &mut enemy.velocity,
            &time
        );

        enemy_transform.translation = new_translation;

        if enemy.can_see_player {
            let player = player.single();
            let angle = (-(player.translation.z - enemy_transform.translation.z))
                .atan2(player.translation.x - enemy_transform.translation.x);
            let rotation = Quat::from_axis_angle(Vec3::Y, angle);

            if !rotation.is_nan() {
                enemy_transform.rotation = rotation;
            }
        } else {
            enemy_transform.rotate_y(time.delta_seconds());
        }

    }
}
