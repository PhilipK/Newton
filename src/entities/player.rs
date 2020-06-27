use crate::components::{Acceleration, Force, Mass, Player, Velocity};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::{Builder, World, WorldExt},
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub fn load_sprite_sheet(world: &World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/player_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/player_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub fn initialize_player(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

    let player = Player {
        forward_thrust_power: 10000.0,
        backwards_thrust_power: 5000.0,
        sideways_thrust_power: 5000.0,
        turn_pr_second: 3.0,
    };
    let mass = Mass::new(1.0);
    let force = Force::new(0.0, 0.0);
    let velocity = Velocity::new(0.0, 0.0);
    let acceleration = Acceleration::new(0.0, 0.0);
    let mut transform = Transform::default();
    //Position the player
    transform.set_translation_xyz(200.0, 500.0, 0.0);

    //Sprite renderer
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // default ship is 0
    };

    world
        .create_entity()
        .with(transform)
        .with(velocity)
        .with(acceleration)
        .with(mass)
        .with(player)
        .with(force)
        .with(sprite_render)
        .build();
}
