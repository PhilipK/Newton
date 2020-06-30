use crate::components::ScoreArea;
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::{Builder, World, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
};

pub fn initialize_score_area(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let score_area = ScoreArea::new(1.0);
    let mut transform = Transform::default();
    //Position the player
    transform.set_translation_xyz(100.0, 200.0, 0.0);

    //Sprite renderer
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(transform)
        .with(score_area)
        .with(sprite_render)
        .build();
}
