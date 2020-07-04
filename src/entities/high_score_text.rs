use amethyst::{
    assets::Loader,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub fn itnitialize_highscore_text(world: &mut World, score: i32) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let transform = UiTransform::new(
        "NEWTON_DEAD_TITLE".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        0.,
        1.,
        500.,
        100.,
    );
    let transform_explain = UiTransform::new(
        "NEWTON_DEAD_EXPLAIN".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        -100.,
        1.,
        600.,
        50.,
    );
    let title = world
        .create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            "YOU DIED".to_string(),
            [1., 1., 1., 1.],
            100.,
        ))
        .build();
    let title_explain = world
        .create_entity()
        .with(transform_explain)
        .with(UiText::new(
            font,
            format!("Score: {} Push S to start", score).to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();
    world.insert(title);
    world.insert(title_explain);
}
