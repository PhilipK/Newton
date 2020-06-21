use crate::components::{Acceleration, Force, Mass, Velocity};
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
            "texture/star.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/star.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub fn initialize_star(
    world: &mut World,
    mass: f32,
    positionx: f32,
    positiony: f32,
    sprite_sheet_handle: Handle<SpriteSheet>,
) {
    let mut transform = Transform::default();
    let mass = Mass::new(mass);
    let force = Force::new(0.0, 0.0);
    let velocity = Velocity::new(10.0, 10.0);
    let acceleration = Acceleration::new(0.0, 0.0);
    //Position the player
    transform.set_translation_xyz(positionx, positiony, 0.0);

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
        .with(force)
        .with(sprite_render)
        .build();
}
