use crate::{
    entities::star,
    resources::SpriteResource,
    systems::{BOX_X_MAX, BOX_Y_MAX},
};
use amethyst::shred::{ReadExpect, World};
use amethyst_core::math::Vector2;
use rand::Rng;

fn generate_meteors(world: &mut World, sprite: &SpriteResource) {
    let mut rng = rand::thread_rng();
    let meteor_number: u32 = rng.gen_range(1, 30);
    for _i in 0..meteor_number {
        let pos_x: f32 = rng.gen_range(0.0, BOX_X_MAX);
        let pos_y: f32 = rng.gen_range(0.0, BOX_Y_MAX);
        let vel_x: f32 = rng.gen_range(-200.0, 200.0);
        let vel_y: f32 = rng.gen_range(-200.0, 200.0);
        star::initialize_star(
            world,
            10.0,
            16.0,
            pos_x,
            pos_y,
            vel_x,
            vel_y,
            sprite.meteor_sprite_sheet_handle.clone().unwrap(),
            4,
            2,
        );
    }
}

fn generate_solar_systems(world: &mut World, sprite: &SpriteResource) {
    let mut rng = rand::thread_rng();
    let star_numbers: u32 = rng.gen_range(5, 10);
    for _i in 0..star_numbers {
        let pos_x: f32 = rng.gen_range(0.0, BOX_X_MAX);
        let pos_y: f32 = rng.gen_range(0.0, BOX_Y_MAX);
        let vel_x: f32 = rng.gen_range(-200.0, 200.0);
        let vel_y: f32 = rng.gen_range(-200.0, 200.0);
        star::initialize_star(
            world,
            1000000.0,
            64.0,
            pos_x,
            pos_y,
            vel_x,
            vel_y,
            sprite.star_sprite_sheet_handle.clone().unwrap(),
            4,
            5,
        );

        let planet_numbers: u32 = rng.gen_range(0, 6);
        let sun_pos = Vector2::new(pos_x, pos_y);
        for _j in 0..planet_numbers {
            let planet_pos_x: f32 = rng.gen_range(-300.0, 200.0);
            let planet_pos_y: f32 = rng.gen_range(-300.0, 200.0);
            let planet_pos = sun_pos + Vector2::new(planet_pos_x, planet_pos_y);
            let planet_to_sun: Vector2<f32> = sun_pos - planet_pos;
            let tangent_to_sun = Vector2::new(-planet_to_sun.y, planet_to_sun.x) * 0.5;
            star::initialize_star(
                world,
                10000.0,
                32.0,
                planet_pos.x,
                planet_pos.y,
                tangent_to_sun.x,
                tangent_to_sun.y,
                sprite.earth_sprite_sheet_handle.clone().unwrap(),
                7,
                5,
            );
        }
    }
}

pub fn generate_world(world: &mut World) {
    let sprite = {
        let s: ReadExpect<SpriteResource> = world.system_data();
        s.clone()
    };
    generate_meteors(world, &sprite);

    generate_solar_systems(world, &sprite);
}
