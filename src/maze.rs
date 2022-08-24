use bevy::prelude::*;
use crate::{
    AppState, collision, assets::GameAssets, ingame, component_adder::{AnimationLink, ComponentAdder},
};
use bevy::gltf::Gltf;
use rand::{random, Rng};

#[derive(Component)]
pub struct MazeMarker {
    pub spawned: bool,
    pub aabb: collision::WorldAabb,
}

#[derive(Component)]
pub struct CornStalk {
    pub is_harvested: bool,
    pub animation_set: bool,
    pub random: f32,
}

pub struct MazePlugin;
impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(animate_corn)
                .with_system(spawn_corn)
        );
    }
}

fn animate_corn( 
    mut corns: Query<(&mut CornStalk, &AnimationLink)>,
    mut animations: Query<&mut AnimationPlayer>,
    game_assets: ResMut<GameAssets>,
) {
    for (mut corn_stalk, animation_link) in &mut corns {
        if let Some(animation_entity) = animation_link.entity {
            if let Ok(mut animation) = animations.get_mut(animation_entity) {
                if !corn_stalk.animation_set {
                    animation.play(game_assets.corn_sway.clone_weak()).repeat();
                    animation.set_speed(corn_stalk.random);
                    animation.set_elapsed(corn_stalk.random);
                    corn_stalk.animation_set = true;
                }

                if animation.is_paused() {
                    animation.resume();
                }
            }
        }
    }
}

fn spawn_corn(
    mut commands: Commands,
    mut maze_planes: Query<(&mut MazeMarker, &mut Visibility)>,
    assets_gltf: Res<Assets<Gltf>>,
    game_assets: Res<GameAssets>,
    mut component_adder: ResMut<ComponentAdder>,
) {
    let maze_thickness = 1.0;
    let corn_height = 80.0;
    let corn_thickness = 0.8;

    if let Some(gltf) = assets_gltf.get(&game_assets.corn_stalk.clone()) {
        let mut rng = rand::thread_rng();
        for (mut maze_plane, mut visibility) in &mut maze_planes {
            if maze_plane.spawned { continue; }
            println!("spawning maze things");

            let rows = ((maze_plane.aabb.max.x - maze_plane.aabb.min.x) / maze_thickness) as usize;
            let columns = ((maze_plane.aabb.max.z - maze_plane.aabb.min.z) / maze_thickness) as usize;

            println!("rows {} columns {} aabb: {:?}", rows, columns, maze_plane.aabb);

            for row in 0..rows {
                for column in 0..columns {
                    let x = maze_plane.aabb.min.x + ((row as f32 + 0.5) * maze_thickness);
                    let z = maze_plane.aabb.min.z + ((column as f32 + 0.5) * maze_thickness);
                    commands .spawn_bundle(SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        transform: {
                            let mut t = Transform::from_xyz(x, 0.0, z);
//                          t.scale.y = corn_height;
//                          t.scale.x = corn_thickness;
//                          t.scale.z = corn_thickness;
                            t
                        },
                        ..default()
                    })
                    .insert(collision::Collidable {
                        aabb: collision::WorldAabb {
                            min: Vec3::new(x - corn_thickness, 0.0, z - corn_thickness),
                            max: Vec3::new(x + corn_thickness, 0.0, z + corn_thickness),
                        },
                    })
                    .insert(AnimationLink {
                        entity: None
                    })
                    .insert(ingame::CleanupMarker)
                    .insert(CornStalk {
                        is_harvested: false,
                        animation_set: false,
                        random: rng.gen_range(0.2..0.5),
                    });
                }
            }
            visibility.is_visible = false; // hide the plane underneath the corn
            maze_plane.spawned = true;
        }
    }

    component_adder.has_linked = false;
}
