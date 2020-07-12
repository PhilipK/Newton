use crate::newton::TitleScreen;
use crate::resources::initialize_audio;
use crate::resources::sprite::initialise_sprite_resource;

use amethyst::prelude::*;

use amethyst::{
    assets::ProgressCounter,
    prelude::{GameData, SimpleState, SimpleTrans, StateData},
};

#[derive(Default)]
pub struct LoadState {
    progress_counter: ProgressCounter,
}

impl LoadState {}

impl<'a, 'b> SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_sprite_resource(world, &mut self.progress_counter);
        initialize_audio(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            println!("done loading");
            return Trans::Push(Box::new(TitleScreen::default()));
        }
        Trans::None
    }
}
