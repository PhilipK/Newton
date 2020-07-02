use amethyst::ecs::ReadExpect;
use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, AudioSink, Source, SourceHandle, WavFormat},
    ecs::{World, WorldExt},
};

const SCORE_SOUND: &str = "audio/score.wav";
const SCORE_TICK_SOUND: &str = "audio/score_tick.wav";
const THRUST_SOUND: &str = "audio/thrust.wav";

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub score_tick_sfx: SourceHandle,
    pub thrust_sfx: SourceHandle,
    pub thrust_sink: AudioSink,
}

fn load_audio_tracks(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, WavFormat, (), &world.read_resource())
}

pub fn initialize_audio(world: &mut World) {
    let sound_effects = {
        let output: ReadExpect<Output> = world.system_data();
        let loader = world.read_resource::<Loader>();
        let thrust_sfx = load_audio_tracks(&loader, world, THRUST_SOUND);
        let output_un = &output;
        let mut sink = AudioSink::new(output_un);
        sink.set_volume(0.2);
        let sound = Sounds {
            score_sfx: load_audio_tracks(&loader, world, SCORE_SOUND),
            score_tick_sfx: load_audio_tracks(&loader, world, SCORE_TICK_SOUND),
            thrust_sfx: thrust_sfx,
            thrust_sink: sink,
        };
        sound
    };
    world.insert(sound_effects);
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
