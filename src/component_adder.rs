use crate::{collision, maze, combine};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use uuid::Uuid;

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
    if component_adder.has_linked {
        return;
    }

    // need to wait until things are actually placed in the world
    if component_adder.frame_cooldown < 2 {
        return;
    }

    for (mut link, children) in &mut animation_links {
        let is_none = link.entity.is_none();
        if is_none  {
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
