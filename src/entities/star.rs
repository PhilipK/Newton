use crate::components::{Acceleration, Force, Gravity, Mass, Planet, SimpleAnimation, Velocity};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::{Builder, World, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
};

pub fn initialize_star(
    world: &mut World,
    mass: f32,
    radius: f32,
    positionx: f32,
    positiony: f32,
    velocityx: f32,
    velocityy: f32,
    sprite_sheet_handle: Handle<SpriteSheet>,
    number_of_sprites: usize,
) {
    let radius_multiplyer = 0.5;
    let mut transform = Transform::default();
    let mass_comp = Mass::new(mass);
    let force = Force::new(0.0, 0.0);
    let velocity = Velocity::new(velocityx, velocityy);
    let acceleration = Acceleration::new(0.0, 0.0);
    let planet = Planet::new(radius * radius_multiplyer);
    //Position the player
    transform.set_translation_xyz(positionx, positiony, 0.0);

    //Sprite renderer
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // default ship is 0
    };

    let mut bundle = world
        .create_entity()
        .with(transform)
        .with(velocity)
        .with(acceleration)
        .with(SimpleAnimation::new(number_of_sprites, 5))
        .with(mass_comp)
        .with(force)
        .with(planet)
        .with(sprite_render);
    if mass > 999.0 {
        bundle = bundle.with(Gravity {});
    }
    bundle.build();
}
