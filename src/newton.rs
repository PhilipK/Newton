use crate::entities::player;
use crate::entities::star;
use crate::playercamera;

use amethyst::{
    assets::Handle,
    prelude::{GameData, SimpleState, StateData},
    renderer::SpriteSheet,
};

#[derive(Default)]
pub struct Newton {
    player_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    star_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Newton {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.player_sprite_sheet_handle
            .replace(player::load_sprite_sheet(world));
        self.star_sprite_sheet_handle
            .replace(star::load_sprite_sheet(world));
        player::initialize_player(world, self.player_sprite_sheet_handle.clone().unwrap());
        star::initialize_star(world, self.star_sprite_sheet_handle.clone().unwrap());
        playercamera::initialize_camera(world);
        // initialise_scoreboard(world);
    }
}
