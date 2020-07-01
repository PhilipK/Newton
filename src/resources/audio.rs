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
    pub thrust_sink: Option<AudioSink>,
}

fn load_audio_tracks(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, WavFormat, (), &world.read_resource())
}

pub fn initialize_audio(world: &mut World) {
    let sound_effects = {
        let loader = world.read_resource::<Loader>();

        let sound = Sounds {
            score_sfx: load_audio_tracks(&loader, world, SCORE_SOUND),
            score_tick_sfx: load_audio_tracks(&loader, world, SCORE_TICK_SOUND),
            thrust_sfx: load_audio_tracks(&loader, world, THRUST_SOUND),
            thrust_sink: None,
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

// pub fn play_thrust_sound(
//     sounds: &mut Sounds,
//     storage: &AssetStorage<Source>,
//     output: Option<&Output>,
// ) {
//     if let Some(ref output) = output.as_ref() {
//         if let None = sounds.thrust_sink {
//             sounds.thrust_sink = Some(AudioSink::new(&output))
//         }
//         if let Some(sink) = sounds.thrust_sink {
//             if let Some(sound) = storage.get(&sounds.thrust_sfx) {
                
//                 sink.play();
//             }
//         }
//     }
// }

// pub fn pause_thrust_sound(
//     sounds: &Sounds,
//     storage: &AssetStorage<Source>,
//     output: Option<&Output>,
// ) {
//     if let Some(ref output) = output.as_ref() {
//         if let Some(sound) = storage.get(&sounds.thrust_sfx) {
//             output.play_once(sound, 1.0);
//         }
//     }
// }

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
