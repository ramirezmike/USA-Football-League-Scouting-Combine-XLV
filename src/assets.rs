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
    pub enemy: Handle<Gltf>,
    pub person_dive: Handle<AnimationClip>,
    pub person_run: Handle<AnimationClip>,
    pub person_idle: Handle<AnimationClip>,
    pub maze: Handle<Gltf>,
    pub football: Handle<Gltf>,
    pub corn_stalk: Handle<Gltf>,
    pub corn_stalk_material: Handle<StandardMaterial>,
    pub corn_sway: Handle<AnimationClip>,
    pub combine: Handle<Gltf>,
    pub combine_drive: Handle<AnimationClip>,

    pub blip: Handle<AudioSource>,
    pub titlescreen: Handle<AudioSource>,

    pub title_screen_background: asset_loading::GameTexture,
    pub title_screen_logo: asset_loading::GameTexture,
    pub bevy_icon: asset_loading::GameTexture,
    pub bill_icon: asset_loading::GameTexture,
    pub will_icon: asset_loading::GameTexture,

    pub blood: asset_loading::GameTexture,
    pub blood_mesh: Handle<Mesh>,

    pub bill_person: Handle<Gltf>,
    pub will_person: Handle<Gltf>,
    pub host_idle: Handle<AnimationClip>,
    pub host_talk: Handle<AnimationClip>,
    pub host_look_left: Handle<AnimationClip>,
    pub host_look_right: Handle<AnimationClip>,
    pub host_look_left_talk: Handle<AnimationClip>,
    pub host_look_right_talk: Handle<AnimationClip>,
    pub will_material: asset_loading::GameTexture,

    pub will_camera: Handle<Image>,
    pub bill_camera: Handle<Image>,
}
