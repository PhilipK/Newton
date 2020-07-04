use amethyst::{
    assets::Loader,
    ecs::prelude::Entity,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub fn itnitialize_title_text(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let transform = UiTransform::new(
        "NEWTON_TITLE".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        0.,
        1.,
        500.,
        100.,
    );
    let transform_explain = UiTransform::new(
        "NEWTON_EXPLAIN".to_string(),
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
            "NEWTON".to_string(),
            [1., 1., 1., 1.],
            100.,
        ))
        .build();
    let title_explain = world
        .create_entity()
        .with(transform_explain)
        .with(UiText::new(
            font,
            "Push W to start".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();
    world.insert(title);
    world.insert(title_explain);
}
