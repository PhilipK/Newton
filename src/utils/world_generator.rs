use crate::{entities::star, resources::SpriteResource};
use amethyst::shred::{ReadExpect, World};

pub fn generate_world(world: &mut World) {
    let sprite = {
        let s: ReadExpect<SpriteResource> = world.system_data();
        s.clone()
    };
    let meteor_number = 20;
    for i in 0..meteor_number {
        star::initialize_star(
            world,
            10.0,
            16.0,
            (10000.0 / meteor_number as f32) * (i as f32),
            (10000.0 / meteor_number as f32) * (i as f32),
            0.0,
            0.0,
            sprite.meteor_sprite_sheet_handle.clone().unwrap(),
            4,
            1,
        );
    }

    star::initialize_star(
        world,
        1000000.0,
        64.0,
        0.0,
        0.0,
        -100.0,
        0.0,
        sprite.star_sprite_sheet_handle.clone().unwrap(),
        4,
        5,
    );

    star::initialize_star(
        world,
        1000000.0,
        64.0,
        -200.0,
        0.0,
        -100.0,
        0.0,
        sprite.star_sprite_sheet_handle.clone().unwrap(),
        4,
        5,
    );
    star::initialize_star(
        world,
        1000000.0,
        64.0,
        -1200.0,
        -1200.0,
        400.0,
        400.0,
        sprite.star_sprite_sheet_handle.clone().unwrap(),
        4,
        5,
    );

    star::initialize_star(
        world,
        1000000.0,
        64.0,
        1200.0,
        1200.0,
        -400.0,
        -400.0,
        sprite.star_sprite_sheet_handle.clone().unwrap(),
        4,
        5,
    );

    star::initialize_star(
        world,
        1000000.0,
        62.0,
        900.0,
        900.0,
        100.0,
        0.0,
        sprite.star_sprite_sheet_handle.clone().unwrap(),
        4,
        5,
    );
    star::initialize_star(
        world,
        1000.0,
        32.0,
        800.0,
        800.0,
        100.0,
        -80.0,
        sprite.earth_sprite_sheet_handle.clone().unwrap(),
        7,
        5,
    );
    star::initialize_star(
        world,
        1000.0,
        32.0,
        100.0,
        100.0,
        -100.0,
        80.0,
        sprite.earth_sprite_sheet_handle.clone().unwrap(),
        7,
        5,
    );
}
