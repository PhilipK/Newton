use crate::components::Player;
use crate::entities::player;
use crate::entities::star;
use crate::playercamera;
use amethyst::ecs::Entities;
use amethyst::ecs::{Join, ReadStorage};
use amethyst::input::{is_key_down, VirtualKeyCode};

use amethyst::prelude::*;

use amethyst::{
    assets::Handle,
    prelude::{GameData, SimpleState, SimpleTrans, StateData},
    renderer::SpriteSheet,
};

#[derive(Default)]
pub struct Newton {
    player_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    star_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    earth_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    meteor_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl Newton {
    fn new() -> Self {
        Newton {
            player_sprite_sheet_handle: Option::None,
            star_sprite_sheet_handle: Option::None,
            earth_sprite_sheet_handle: Option::None,
            meteor_sprite_sheet_handle: Option::None,
        }
    }
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

        let meteor_number = 10;
        for i in 0..meteor_number {
            star::initialize_star(
                world,
                10.0,
                16.0,
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
            64.0,
            200.0,
            200.0,
            -100.0,
            0.0,
            self.star_sprite_sheet_handle.clone().unwrap(),
        );

        star::initialize_star(
            world,
            1000000.0,
            62.0,
            700.0,
            700.0,
            100.0,
            0.0,
            self.star_sprite_sheet_handle.clone().unwrap(),
        );
        star::initialize_star(
            world,
            1000.0,
            32.0,
            800.0,
            800.0,
            100.0,
            -80.0,
            self.earth_sprite_sheet_handle.clone().unwrap(),
        );
        star::initialize_star(
            world,
            1000.0,
            32.0,
            100.0,
            100.0,
            -100.0,
            80.0,
            self.earth_sprite_sheet_handle.clone().unwrap(),
        );
        playercamera::initialize_camera(world);
        // initialise_scoreboard(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let system_data: Entities = data.world.system_data();
        for entity in (&system_data).join() {
            system_data.delete(entity);
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let system_data: ReadStorage<Player> = data.world.system_data();
        for (player) in (&system_data).join() {
            if (player.is_dead) {
                return Trans::Replace(Box::new((Newton::new())));
            }
        }

        Trans::None
    }
}
