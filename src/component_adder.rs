use crate::{collision, maze, combine, assets::GameAssets, ingame, other_persons, game_state};
use bevy::prelude::*;
use bevy::gltf::Gltf;
use bevy::render::primitives::Aabb;
use uuid::Uuid;
use std::f32::consts::{FRAC_PI_2, TAU};
use bevy::render::{
    view::RenderLayers,
};

pub struct ComponentAdderPlugin;
impl Plugin for ComponentAdderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ComponentAdder::default())
            .add_system(link_animations)
            .add_system(add_components);
    }
}

#[derive(Default)]
pub struct ComponentAdder {
    has_added: bool,
    pub has_linked: bool,
    frame_cooldown: usize,
}

impl ComponentAdder {
    pub fn reset(&mut self) {
        self.has_added = false;
        self.has_linked = false;
        self.frame_cooldown = 0;
    }
}

fn add_components(
    mut commands: Commands,
    mut items: Query<(Entity, &Aabb, &GlobalTransform, &mut Name, &mut Visibility), With<Parent>>,
    mut component_adder: ResMut<ComponentAdder>,
    mut game_state: ResMut<game_state::GameState>,
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    if component_adder.has_added {
        return;
    }
    component_adder.frame_cooldown += 1;

    // need to wait until things are actually placed in the world
    if component_adder.frame_cooldown < 2 {
        return;
    }

    for (entity, aabb, global_transform, mut name, mut visibility) in items.iter_mut() {
        let mut change_name = false;
        if name.as_str().contains("dynamic_collide") {
            commands
                .entity(entity)
                .insert(collision::DynamicCollidable);

            change_name = true;
        }
        if name.as_str().contains("collidable") {
            let matrix = global_transform.compute_matrix();
            commands
                .entity(entity)
                .insert(collision::Collidable {
                    aabb: collision::WorldAabb {
                        min: matrix.transform_point3(aabb.min().into()),
                        max: matrix.transform_point3(aabb.max().into()),
                    },
                });

            change_name = true;
        }

        if name.as_str().contains("maze") {
            if !game_state.corn_spawned {
                let matrix = global_transform.compute_matrix();
                commands
                    .entity(entity)
                    .insert(maze::MazeMarker {
                        spawned: false,
                        aabb: collision::WorldAabb {
                            min: matrix.transform_point3(aabb.min().into()),
                            max: matrix.transform_point3(aabb.max().into()),
                        },
                    });

                println!("found maze");
            }
            visibility.is_visible = false;
            change_name = true;
        }

        if name.as_str().contains("combine_blade") {
            let matrix = global_transform.compute_matrix();
            commands
                .entity(entity)
                .insert(combine::CombineBlade);
            visibility.is_visible = false;

            println!("found combine blade");
            change_name = true;
        }

        if name.as_str().contains("bill") {
            if let Some(gltf) = assets_gltf.get(&game_assets.bill_person.clone()) {
                let matrix = global_transform.compute_matrix();
                commands.spawn_bundle(SceneBundle {
                            scene: gltf.scenes[0].clone(),
                            transform: {
                                let mut t = Transform::from_translation(matrix.transform_point3(aabb.center.into()));
                                t.translation.y = 0.0;
                                t.rotation = Quat::from_rotation_y(TAU * 0.5);
                                t
                            },
                            ..default()
                        })
                        .insert(other_persons::BillPerson)
                        .insert(AnimationLink {
                            entity: None
                        })
                        .insert(ingame::CleanupMarker);
                visibility.is_visible = false;
            }

            change_name = true;
        }

        if name.as_str().contains("will") {
            if let Some(gltf) = assets_gltf.get(&game_assets.will_person.clone()) {
                let matrix = global_transform.compute_matrix();
                let first_pass_layer = RenderLayers::layer(1);

                commands.spawn_bundle(SceneBundle {
                            scene: gltf.scenes[0].clone(),
                            transform: {
                                let mut t = Transform::from_translation(matrix.transform_point3(aabb.center.into()));
                                t.translation.y = 0.0;
                                t.rotation = Quat::from_rotation_y(TAU * 0.5);
                                t
                            },
                            ..default()
                        })
                        .insert(other_persons::WillPerson)
                        .insert(first_pass_layer)
                        .insert(AnimationLink {
                            entity: None
                        })
                        .insert(ingame::CleanupMarker);
                visibility.is_visible = false;
            }

            change_name = true;
        }

        if change_name {
            *name = Name::new(Uuid::new_v4().to_string());
        }
    }

    component_adder.has_added = true;
}

#[derive(Component)]
pub struct AnimationLink {
    pub entity: Option::<Entity>,
}

fn link_animations(
    mut component_adder: ResMut<ComponentAdder>,
    mut animation_links: Query<(&mut AnimationLink, &Children)>,
    animations: Query<(&Parent, Entity), With<AnimationPlayer>>,
) {
    for (mut link, children) in &mut animation_links {
        let is_none = link.entity.is_none();
        if is_none {
            for child in children {
                for (parent, entity) in &animations {
                    if parent.get() == *child {
                        link.entity = Some(entity);
                    }
                }
            }
        }
    }

    component_adder.has_linked = true;
}
