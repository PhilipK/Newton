use crate::components::{Acceleration, Force, Mass, Player, Velocity};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::{Builder, World, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
};

pub fn initialize_player(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let player = Player {
        forward_thrust_power: 10000.0,
        backwards_thrust_power: 3333.0,
        sideways_thrust_power: 5000.0,
        turn_pr_second: 3.0,
        is_dead: false,
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
