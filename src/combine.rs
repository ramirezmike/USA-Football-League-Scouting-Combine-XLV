use bevy::prelude::*;
use crate::{
    AppState, maze::CornStalk, assets::GameAssets,
};

pub struct CombinePlugin;
impl Plugin for CombinePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(handle_corn_collision)
        );
    }
}

#[derive(Component)]
pub struct Combine;

const CORN_CUT_DISTANCE: f32 = 0.7;
fn handle_corn_collision( 
    mut corns: Query<(&mut CornStalk, &mut Transform), Without<Combine>>,
    combines: Query<&Transform, Without<CornStalk>>,
) {
    for combine_transform in &combines {
        for (mut corn, mut corn_transform) in &mut corns {
            if corn.is_harvested { continue; }

            if corn_transform.translation.distance(combine_transform.translation) < CORN_CUT_DISTANCE {
                corn_transform.scale.y = 0.1;
                corn.is_harvested = true;
            }
        }
    }
}

