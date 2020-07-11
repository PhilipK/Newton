use crate::components::Destroyable;
use crate::components::ScoreArea;
use crate::playercamera::{CAMERA_HEIGHT, CAMERA_WIDTH};
use crate::systems::score_system::TIME_TO_SCORE;
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::{Builder, World, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
};

pub fn initialize_score_area(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let score_area = ScoreArea::new(TIME_TO_SCORE);
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 0.0);

    //Sprite renderer
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(transform)
        .with(score_area)
        .with(Destroyable {})
        .with(sprite_render)
        .build();
}
