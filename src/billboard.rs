use bevy::prelude::*;
use crate::{game_camera};

pub struct BillboardPlugin;
impl Plugin for BillboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_billboards)
           .add_system(animate_billboards);
    }
}

#[derive(Component)]
pub struct Billboard;

fn handle_billboards(
    mut billboards: Query<&mut Transform, With<Billboard>>,
    camera: Query<&Transform, (With<game_camera::PanOrbitCamera>, Without<Billboard>)>,
) {
    if let Ok(camera) = camera.get_single() {
        for mut billboard in billboards.iter_mut() {
            println!("ah");
        //    billboard.look_at(camera.translation, Vec3::Y);
        }
    }
}

pub fn animate_billboards(
    mut commands: Commands,
    mut billboards: Query<(&Billboard, &mut Transform, &Handle<StandardMaterial>, Entity)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (billboard, mut transform, material, entity) in &mut billboards {
        transform.rotate(Quat::from_rotation_x(time.delta_seconds()));
        transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
        transform.scale *= 1.0 - (time.delta_seconds() * 0.1);

//      let target = transform
//          .translation
//          .lerp(Vec3::Y, time.delta_seconds() * 0.3);
//      if !target.is_nan() {
//          transform.translation = target;
//      }

        transform.translation.y += time.delta_seconds() * 2.5;

        let mut despawn_entity = true; // if the material doesn't exist, just despawn
//      if let Some(material) = materials.get_mut(material) {
//          let a = material.base_color.a();
//          if a > 0.0 {
//              despawn_entity = false;
//              material.base_color.set_a(a - (time.delta_seconds() * 1.25));
//          }
//      }

        if despawn_entity {
//            commands.entity(entity).despawn_recursive();
        }
    }
}

