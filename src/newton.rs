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
    earth_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    meteor_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Newton {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.player_sprite_sheet_handle
            .replace(player::load_sprite_sheet(world));
        self.star_sprite_sheet_handle
            .replace(star::load_sprite_sheet(world, "star"));
        self.earth_sprite_sheet_handle
            .replace(star::load_sprite_sheet(world, "earth"));
        self.meteor_sprite_sheet_handle
            .replace(star::load_sprite_sheet(world, "meteor"));
        player::initialize_player(world, self.player_sprite_sheet_handle.clone().unwrap());

        let meteor_number = 1000;
        for i in 0..meteor_number {
            star::initialize_star(
                world,
                10.0,
                (10000.0 / meteor_number as f32) * (i as f32),
                (10000.0 / meteor_number as f32) * (i as f32),
                0.0,
                0.0,
                self.meteor_sprite_sheet_handle.clone().unwrap(),
            );
        }

        star::initialize_star(
            world,
            1000000.0,
            200.0,
            200.0,
            -100.0,
            0.0,
            self.star_sprite_sheet_handle.clone().unwrap(),
        );

        star::initialize_star(
            world,
            1000000.0,
            700.0,
            700.0,
            100.0,
            0.0,
            self.star_sprite_sheet_handle.clone().unwrap(),
        );
        star::initialize_star(
            world,
            1000.0,
            800.0,
            800.0,
            100.0,
            -80.0,
            self.earth_sprite_sheet_handle.clone().unwrap(),
        );
        star::initialize_star(
            world,
            1000.0,
            100.0,
            100.0,
            -100.0,
            80.0,
            self.earth_sprite_sheet_handle.clone().unwrap(),
        );
        playercamera::initialize_camera(world);
        // initialise_scoreboard(world);
    }
}
