use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_kira_audio::{AudioApp, AudioChannel, AudioPlugin, AudioSource};
use std::marker::PhantomData;

pub struct GameAudioPlugin;
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<MusicChannel>()
            .add_audio_channel::<SoundChannel>()
            .add_audio_channel::<TalkChannel>()
            .add_plugin(AudioPlugin);
    }
}

pub struct MusicChannel;
pub struct SoundChannel;
pub struct TalkChannel;

#[derive(SystemParam)]
pub struct GameAudio<'w, 's> {
    music_channel: Res<'w, AudioChannel<MusicChannel>>,
    sound_channel: Res<'w, AudioChannel<SoundChannel>>,
    talk_channel: Res<'w, AudioChannel<TalkChannel>>,

    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'w, 's> GameAudio<'w, 's> {
    pub fn play_bgm(&mut self, handle: &Handle<AudioSource>) {
        self.music_channel.stop();
        self.music_channel.set_volume(0.5);
        self.music_channel.play_looped(handle.clone());
    }

    pub fn play_bgm_once(&mut self, handle: &Handle<AudioSource>) {
        self.music_channel.stop();
        self.music_channel.set_volume(0.5);
        self.music_channel.play(handle.clone());
    }

    pub fn stop_bgm(&mut self) {
        self.music_channel.stop();
    }

    pub fn play_sfx(&mut self, handle: &Handle<AudioSource>) {
        self.sound_channel.set_volume(0.2);
        self.sound_channel.play(handle.clone());
    }
    pub fn play_talk(&mut self, handle: &Handle<AudioSource>) {
        self.talk_channel.set_volume(0.2);
        self.talk_channel.play(handle.clone());
    }
}
