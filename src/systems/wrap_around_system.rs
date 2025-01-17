use crate::components::*;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

#[derive(SystemDesc)]
pub struct WrapAroundSystem;

pub const SPRITE_SIZE: f32 = 512.0;
pub const NUMBER_OF_TILES: f32 = 15.0;
pub const BOX_X_MIN: f32 = 0.;
pub const BOX_Y_MIN: f32 = 0.;
pub const BOX_WIDTH: f32 = SPRITE_SIZE * NUMBER_OF_TILES;
pub const BOX_HEIGHT: f32 = SPRITE_SIZE * NUMBER_OF_TILES;
pub const BOX_X_MAX: f32 = BOX_WIDTH;
pub const BOX_Y_MAX: f32 = BOX_HEIGHT;

impl<'s> System<'s> for WrapAroundSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Velocity>);

    fn run(&mut self, (mut transforms, velocities): Self::SystemData) {
        for (transform, _velocity) in (&mut transforms, &velocities).join() {
            let (x, y) = {
                let trans = transform.translation();
                (trans.x, trans.y)
            };
            if x < BOX_X_MIN {
                transform.set_translation_x(x + BOX_WIDTH);
            }
            if y < BOX_Y_MIN {
                transform.set_translation_y(y + BOX_HEIGHT);
            }
            if x > BOX_X_MAX {
                transform.set_translation_x(x - BOX_WIDTH);
            }
            if y > BOX_Y_MAX {
                transform.set_translation_y(y - BOX_HEIGHT);
            }
        }
    }
}
