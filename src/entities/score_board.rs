use amethyst::{
    assets::Loader,
    ecs::prelude::Entity,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

#[derive(Default)]
pub struct ScoreBoard {
    pub score: i32,
}

pub struct ScoreText {
    pub score: Entity,
    pub timer: Entity,
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
        .with(UiText::new(font.clone(), "0".to_string(), [1., 1., 1., 1.], 50.))
        .build();

    let transform = UiTransform::new(
        "TIMER".to_string(),
        Anchor::BottomMiddle,
        Anchor::BottomMiddle,
        0.,
        0.,
        1.,
        200.,
        50.,
    );
    let timer = world
        .create_entity()
        .with(transform)
        .with(UiText::new(font, "30.0".to_string(), [1., 1., 1., 1.], 50.))
        .build();

    world.insert(ScoreText { score, timer });
}
