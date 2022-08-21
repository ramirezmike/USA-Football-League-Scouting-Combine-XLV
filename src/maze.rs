use bevy::prelude::*;
use crate::{
    AppState, collision::WorldAabb, assets::GameAssets, ingame,
};

#[derive(Component)]
pub struct MazeMarker {
    pub spawned: bool,
    pub aabb: WorldAabb,
}

#[derive(Component)]
pub struct CornStalk {
    pub is_harvested: bool
}

pub struct MazePlugin;
impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(spawn_corn)
        );
    }
}

fn spawn_corn(
    mut commands: Commands,
    mut maze_planes: Query<(&mut MazeMarker, &mut Visibility)>,
    game_assets: Res<GameAssets>,
) {
    let maze_thickness = 0.5;
    let corn_height = 80.0;
    let corn_thickness = 0.8;

    for (mut maze_plane, mut visibility) in &mut maze_planes {
        if maze_plane.spawned { continue; }
        println!("spawning maze things");

        let rows = ((maze_plane.aabb.max.x - maze_plane.aabb.min.x) / maze_thickness) as usize;
        let columns = ((maze_plane.aabb.max.z - maze_plane.aabb.min.z) / maze_thickness) as usize;

        println!("rows {} columns {} aabb: {:?}", rows, columns, maze_plane.aabb);

        for row in 0..rows {
            for column in 0..columns {
                commands.spawn_bundle(PbrBundle {
                    mesh: game_assets.corn_stalk.clone_weak(),
                    material: game_assets.corn_stalk_material.clone_weak(),
                    transform: {
                        let mut t = Transform::from_xyz(maze_plane.aabb.min.x + ((row as f32 + 0.5) * maze_thickness), 
                                                        0.0, 
                                                        maze_plane.aabb.min.z + ((column as f32 + 0.5) * maze_thickness));
                        t.scale.y = corn_height;
                        t.scale.x = corn_thickness;
                        t.scale.z = corn_thickness;
                        t
                    },
                    ..Default::default()
                })
                .insert(ingame::CleanupMarker)
                .insert(CornStalk {
                    is_harvested: false
                });
            }
        }
        visibility.is_visible = false; // hide the plane underneath the corn
        maze_plane.spawned = true;
    }
}
