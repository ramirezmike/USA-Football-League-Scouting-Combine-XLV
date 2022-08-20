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
    pub fn fit_in(&self, current: &Vec3, new: &mut Vec3, velocity: &mut Vec3) {
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

        let mut temp_new = *current;
//      if !current_aabbs.is_empty() {
//          let aabb = current_aabbs[0];
//          if temp_new.x > aabb.min.x && temp_new.x < aabb.max.x {
//              temp_new.x = current.x;
//          }

//          if temp_new.z > aabb.min.z && temp_new.z < aabb.max.z {
//              temp_new.z = current.z;
//          }
//      } else {
//          temp_new = *current;
//      }

//      // All this allows sliding against walls
//      let x_changed = temp_new.x != new.x;
//      let z_changed = temp_new.z != new.z;
//      let get_sign = |num: f32| {
//          let sign = num.signum();
//          if sign.is_nan() {
//              1.0
//          } else {
//              sign
//          }
//      };

//      match (x_changed, z_changed) {
//          (true, true) => {
//              velocity.x = 0.0;
//              velocity.z = 0.0;
//          }
//          (true, false) => {
//              velocity.z = get_sign(velocity.z) * f32::max(velocity.z.abs(), velocity.x.abs());
//              velocity.x = 0.0;
//          }
//          (false, true) => {
//              velocity.x = get_sign(velocity.x) * f32::max(velocity.z.abs(), velocity.x.abs());
//              velocity.z = 0.0;
//          }
//          _ => (),
//      }

        *new = temp_new;
    }
}

