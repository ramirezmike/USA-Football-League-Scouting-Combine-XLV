use bevy::ecs::system::SystemParam;
use bevy::render::primitives::Aabb;
use bevy::prelude::*;
use crate::{
    LEFT_END, RIGHT_END, BOTTOM_END, TOP_END,
};

#[derive(Component)]
pub struct Collidable {
    pub aabb: WorldAabb,
}

#[derive(Component)]
pub struct DynamicCollidable;

#[derive(Debug, Default, Copy, Clone)]
pub struct WorldAabb {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(SystemParam)]
pub struct Collidables<'w, 's> {
    collidables: Query<'w, 's, &'static Collidable>,
    dynamic_collidables: Query<'w, 's, (Entity, &'static DynamicCollidable)>,
    aabbs: Query<'w, 's, (&'static Aabb, &'static GlobalTransform)>,
}

impl<'w, 's> Collidables<'w, 's> {
    pub fn is_in_collidable(&self, position: &Vec3) -> bool {
        if position.z <= LEFT_END ||
           position.z >= RIGHT_END ||
           position.x <= BOTTOM_END ||
           position.x >= TOP_END {
           return true;
        }

        if self.collidables.iter().count() == 0 && self.dynamic_collidables.iter().count() == 0 {
            return false;
        }

        for collidable in self.collidables.iter() {
            if position.x <= collidable.aabb.max.x
                && position.x >= collidable.aabb.min.x
                && position.z <= collidable.aabb.max.z
                && position.z >= collidable.aabb.min.z
            {
                return true;
            }
        }

        let dynamic_collidables = self.dynamic_collidables
                                      .iter()
                                      .filter_map(|(entity, _)| self.aabbs.get(entity).ok());

        for (aabb, global_transform) in dynamic_collidables {
            let matrix = global_transform.compute_matrix();
            let inverse_matrix = matrix.inverse();
            let min = aabb.min();
            let max = aabb.max();

            let transformed_new = inverse_matrix.transform_point3(*position);

            if transformed_new.x <= max.x
                && transformed_new.x >= min.x
                && transformed_new.z <= max.z
                && transformed_new.z >= min.z
            {
                return true; 
            }
        }

        return false;
    }

    pub fn fit_in(&self, current: &Vec3, new: &mut Vec3, velocity: &mut Vec3, time: &Res<Time>) {
        if self.collidables.iter().count() == 0 && self.dynamic_collidables.iter().count() == 0 {
            return;
        }

        if new.z <= LEFT_END {
            *new = *current;
            *velocity = Vec3::new(velocity.x, velocity.y, velocity.z.abs().max(1.0) * 2.0);
            return; 
        }
        if new.z >= RIGHT_END {
            *new = *current;
            *velocity = Vec3::new(velocity.x, velocity.y, (velocity.z.abs() * -1.0).min(-1.0) * 2.0);
            return; 
        }
        if new.x <= BOTTOM_END {
            *new = *current;
            *velocity = Vec3::new(velocity.x.abs().max(1.0) * 2.0, velocity.y, velocity.z);
            return; 
        }
        if new.x >= TOP_END {
            *new = *current;
            *velocity = Vec3::new((velocity.x.abs() * -1.0).min(-1.0) * 2.0, velocity.y, velocity.z);
            return; 
        }

        let mut is_valid = true;
        let mut current_aabbs = vec![];

        for collidable in self.collidables.iter() {
            if new.x <= collidable.aabb.max.x
                && new.x >= collidable.aabb.min.x
                && new.z <= collidable.aabb.max.z
                && new.z >= collidable.aabb.min.z
            {
                is_valid = false;
                current_aabbs.push(collidable.aabb);
            }
        }

        let dynamic_collidables = self.dynamic_collidables
                                      .iter()
                                      .filter_map(|(entity, _)| self.aabbs.get(entity).ok());

        for (aabb, global_transform) in dynamic_collidables {
            let matrix = global_transform.compute_matrix();
            let inverse_matrix = matrix.inverse();
            let min = aabb.min();
            let max = aabb.max();

            let transformed_new = inverse_matrix.transform_point3(*new);

            if transformed_new.x <= max.x
                && transformed_new.x >= min.x
                && transformed_new.z <= max.z
                && transformed_new.z >= min.z
            {
                // bounce away I guess
                *new = *current;
                *velocity = Vec3::new(-velocity.x, -velocity.y, -velocity.z) * 2.0;
                return; 
            }
        }

        if is_valid {
            return;
        }

        let get_sign = |num: f32| {
            let sign = num.signum();
            if sign.is_nan() {
                1.0
            } else {
                sign
            }
        };

        let mut temp_new = *current;
        for aabb in current_aabbs.iter() {
            let top_normal = (Vec3::new(aabb.max.x, 0.0, aabb.min.z)
                            - Vec3::new(aabb.max.x, 0.0, aabb.max.z)).cross(Vec3::Y).normalize();
            let bottom_normal = (Vec3::new(aabb.min.x, 0.0, aabb.max.z)
                               - Vec3::new(aabb.min.x, 0.0, aabb.min.z)).cross(Vec3::Y).normalize();
            let left_normal = (Vec3::new(aabb.min.x, 0.0, aabb.min.z)
                             - Vec3::new(aabb.max.x, 0.0, aabb.min.z)).cross(Vec3::Y).normalize();
            let right_normal = (Vec3::new(aabb.max.x, 0.0, aabb.max.z)
                              - Vec3::new(aabb.min.x, 0.0, aabb.max.z)).cross(Vec3::Y).normalize();

            let normalized_current = (*new - temp_new).normalize();
            if top_normal.dot(normalized_current) < 0.0 && temp_new.x > aabb.max.x {
                let undesired = (velocity.x.abs().max(1.0) * top_normal) * normalized_current.dot(top_normal);
                *new = temp_new - (undesired * time.delta_seconds());
                velocity.x = 0.0;
                velocity.z = velocity.z + (get_sign(velocity.z) * velocity.x);
            } else if bottom_normal.dot(normalized_current) < 0.0 && temp_new.x < aabb.min.x {
                let undesired = (velocity.x.abs().max(1.0) * bottom_normal) * normalized_current.dot(bottom_normal);
                *new = temp_new - (undesired * time.delta_seconds());
                velocity.x = 0.0;
                velocity.z = velocity.z + (get_sign(velocity.z) * velocity.x);
            } else if left_normal.dot(normalized_current) < 0.0 && temp_new.z < aabb.min.z {
                let undesired = (velocity.z.abs().max(1.0) * left_normal) * normalized_current.dot(left_normal);
                *new = temp_new - (undesired * time.delta_seconds());
                velocity.x = velocity.x + (get_sign(velocity.x) * velocity.z);
                velocity.z = 0.0;
            } else if right_normal.dot(normalized_current) < 0.0 && temp_new.z > aabb.max.z {
                let undesired = (velocity.z.abs().max(1.0) * right_normal) * normalized_current.dot(right_normal);
                *new = temp_new - (undesired * time.delta_seconds());
                velocity.x = velocity.x + (get_sign(velocity.x) * velocity.z);
                velocity.z = 0.0;
            } 
        }

    }
}
