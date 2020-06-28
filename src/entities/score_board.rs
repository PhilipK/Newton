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
pub struct ScoreBoard {
    pub score: i32,
}

pub struct ScoreText {
    pub score: Entity,
}

pub fn initialise_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let transform = UiTransform::new(
        "SCORE".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        0.,
        0.,
        1.,
        200.,
        50.,
    );
    let score = world
        .create_entity()
        .with(transform)
        .with(UiText::new(font, "0".to_string(), [1., 1., 1., 1.], 50.))
        .build();
    world.insert(ScoreText { score });
}
