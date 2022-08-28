use crate::{AppState, game_state, collision, assets::GameAssets, player::Player, ingame,
LEFT_END, RIGHT_END, LEFT_GOAL, RIGHT_GOAL, BOTTOM_END, TOP_END, enemy};
use bevy::prelude::*;
use rand::Rng;
use bevy::gltf::Gltf;
use std::f32::consts::{FRAC_PI_2};

pub struct FootballPlugin;
impl Plugin for FootballPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaunchFootballEvent>()
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(check_for_football_pickup)
                .with_system(handle_launch_football_event)
                .with_system(move_football),
        );
    }
}

#[derive(Component)]
pub struct Football {
    pub has_landed: bool,
    target: Vec3,
    starting_position: Vec3,
    current_movement_time: f32,
}
#[derive(Component)]
pub struct CarriedFootball;
pub struct LaunchFootballEvent;

fn handle_launch_football_event(
    mut commands: Commands,
    mut launch_football_event_reader: EventReader<LaunchFootballEvent>,
    game_assets: Res<GameAssets>,
    collidables: collision::Collidables,
    assets_gltf: Res<Assets<Gltf>>,
    mut game_state: ResMut<game_state::GameState>,
    mut spawn_enemies_event_writer: EventWriter<enemy::SpawnEnemiesEvent>,
) {
    for event in launch_football_event_reader.iter() {
        if let Some(gltf) = assets_gltf.get(&game_assets.football.clone()) {
            let left_side = Vec3::new(6.976, 0.0, -48.0);
            let right_side = Vec3::new(6.976, 0.0, 48.0);

            let position = if game_state.touchdown_on_leftside { right_side } else { left_side };

            let mut target = None;
            let mut rng = rand::thread_rng();
            let z_buffer = ((RIGHT_GOAL - LEFT_GOAL).abs() * 0.25);
            let x_buffer = ((TOP_END - BOTTOM_END).abs() * 0.02);
            let min_z = LEFT_GOAL + z_buffer;
            let max_z = RIGHT_GOAL - z_buffer;
            let min_x = BOTTOM_END + x_buffer;
            let max_x = TOP_END - x_buffer;
            while target.is_none() {
                let potential_position = Vec3::new(rng.gen_range(min_x..max_x), 
                                                   0.0, 
                                                   rng.gen_range(min_z..max_z));
                if !collidables.is_in_collidable(&potential_position) {
                    target = Some(potential_position);
                }
            }

            commands.spawn_bundle(SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        transform: {
                            let mut t = Transform::from_scale(Vec3::splat(3.0));
                            t.translation = position;
                            t
                        },
                        ..default()
                    })
                    .insert(Football {
                        has_landed: false,
                        target: target.unwrap(),
                        starting_position: position,
                        current_movement_time: 0.0,
                    })
                    .insert(ingame::CleanupMarker);
        }
        if !game_state.enemies_spawned {
            spawn_enemies_event_writer.send(enemy::SpawnEnemiesEvent);
            game_state.enemies_spawned = true;
        }
    }
}

const FOOTBALL_PICKUP_DISTANCE: f32 = 1.5;
fn check_for_football_pickup(
    mut commands: Commands,
    footballs: Query<(Entity, &Football, &Transform)>,
    mut player: Query<(Entity, &mut Player, &Transform)>,
    mut carried_footballs: Query<(&CarriedFootball, &mut Visibility, &Parent)>,
) {
    for (football_entity, football, football_transform) in &footballs {
        let (player_entity, mut player, player_transform) = player.single_mut();

        if football_transform.translation.distance(player_transform.translation) < FOOTBALL_PICKUP_DISTANCE {
            player.has_football = true;

            for (_, mut visibility, parent) in &mut carried_footballs {
                if player_entity == parent.get() {
                    visibility.is_visible = true;
                }
            }

            commands.entity(football_entity).despawn_recursive();
        }
    }
}

fn move_football(
    mut footballs: Query<(&mut Football, &mut Transform)>,
    time: Res<Time>,
) {
    let flight_time = 2.0;
    let flight_height = 20.0;

    for (mut football, mut transform) in &mut footballs {
        if !football.has_landed {
            let (target_with_height, start_with_height) 
                = if football.current_movement_time / flight_time <= 0.5 {
                     (Vec3::new(football.target.x, flight_height, football.target.z),
                     football.starting_position)
                  } else {
                     (football.target,
                     (Vec3::new(football.starting_position.x, flight_height, football.starting_position.z)))
                  };
            transform.translation = 
                start_with_height.lerp(target_with_height, football.current_movement_time / flight_time);
            transform.rotate_x(time.delta_seconds());
            transform.rotate_y(time.delta_seconds() / 2.0);
            transform.rotate_z(time.delta_seconds() / 3.0);
            football.current_movement_time += time.delta_seconds();

            if football.current_movement_time >= flight_time {
                football.current_movement_time = 0.0;
                football.has_landed = true;
                transform.rotation = Quat::IDENTITY;
            }
        }
    }
}
