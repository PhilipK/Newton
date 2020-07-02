use amethyst::ecs::ReadExpect;
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};
use amethyst::{assets::Loader, audio::AudioSink};

const SCORE_SOUND: &str = "audio/score.ogg";
const SCORE_TICK_SOUND: &str = "audio/score_tick.ogg";
const THRUST_SOUND: &str = "audio/thrust.ogg";

const MUSIC_TRACKS: &[&str] = &["audio/music.ogg"];

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub score_tick_sfx: SourceHandle,
    pub thrust_sfx: SourceHandle,
    pub thrust_sink: AudioSink,
}

use std::{iter::Cycle, vec::IntoIter};

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_tracks(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialize_audio(world: &mut World) {
    let (sound_effects, music) = {
        let output: ReadExpect<Output> = world.system_data();
        let loader = world.read_resource::<Loader>();
        let thrust_sfx = load_audio_tracks(&loader, world, THRUST_SOUND);
        let output_un = &output;
        let mut sink = AudioSink::new(output_un);
        sink.set_volume(0.25);

        let mut music_sink = world.write_resource::<AudioSink>();
        music_sink.set_volume(0.05); // Music is a bit loud, reduce the volume.
        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_tracks(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        let sound = Sounds {
            score_sfx: load_audio_tracks(&loader, world, SCORE_SOUND),
            score_tick_sfx: load_audio_tracks(&loader, world, SCORE_TICK_SOUND),
            thrust_sfx: thrust_sfx,
            thrust_sink: sink,
        };
        (sound, music)
    };
    world.insert(sound_effects);
    world.insert(music);
}

pub fn play_score_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn play_thrust_sound(sounds: &Sounds, storage: &AssetStorage<Source>) {
    let sink = &sounds.thrust_sink;
    if sink.empty() {
        if let Some(sound) = storage.get(&sounds.thrust_sfx) {
            let _ = sink.append(sound);
        }
    }
    if sink.is_paused() {
        sink.play();
    }
}

pub fn pause_thrust_sound(sounds: &Sounds) {
    if !sounds.thrust_sink.empty() && !sounds.thrust_sink.is_paused() {
        let _ = sounds.thrust_sink.pause();
    }
}

pub fn play_score_tick_sound(
    sounds: &Sounds,
    storage: &AssetStorage<Source>,
    output: Option<&Output>,
) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_tick_sfx) {
            output.play_once(sound, 0.5);
        }
    }
}
