use crate::asset_loading;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameAssets::default());
    }
}

#[derive(Default)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub person: Handle<Gltf>,
    pub person_run: Handle<AnimationClip>,
    pub person_idle: Handle<AnimationClip>,
    pub maze: Handle<Gltf>,
    pub corn_stalk: Handle<Mesh>,
    pub corn_stalk_material: Handle<StandardMaterial>,
    pub combine: Handle<Gltf>,
    pub combine_drive: Handle<AnimationClip>,

    pub blip: Handle<AudioSource>,
    pub titlescreen: Handle<AudioSource>,

    pub title_screen_background: asset_loading::GameTexture,
    pub title_screen_logo: asset_loading::GameTexture,
}
