use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(Component)]
pub struct Collidable {
    pub aabb: WorldAabb,
}

#[derive(Debug, Default)]
pub struct WorldAabb {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(SystemParam)]
pub struct Collidables<'w, 's> {
    collidables: Query<'w, 's, &'static Collidable>,
}

impl<'w, 's> Collidables<'w, 's> {
    pub fn fit_in(&self, current: &Vec3, new: &mut Vec3, velocity: &mut Vec3, time: &Res<Time>) {
        if self.collidables.iter().count() == 0 {
            return;
        }

        let mut is_valid = true;
        let mut current_aabbs = vec![];
        for collidable in self.collidables.iter() {
            let aabb = &collidable.aabb;

            if new.x <= aabb.max.x
                && new.x >= aabb.min.x
                && new.z <= aabb.max.z
                && new.z >= aabb.min.z
            {
                is_valid = false;
                current_aabbs.push(aabb);
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

