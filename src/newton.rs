use crate::player;
use crate::playercamera;
use amethyst::core::timing::Time;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

#[derive(Default)]
pub struct Newton {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Newton {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.sprite_sheet_handle
            .replace(player::load_sprite_sheet(world));
        player::initialize_player(world, self.sprite_sheet_handle.clone().unwrap());
        playercamera::initialize_camera(world);
        // initialise_scoreboard(world);
    }
}
