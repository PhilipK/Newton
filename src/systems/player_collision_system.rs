use crate::components::Planet;
use crate::components::Player;
use crate::utils::distance_squared;

use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage};
// use amethyst::ecs::{Write};
// use amethyst::renderer::debug_drawing::DebugLines;
// use amethyst_core::math::Point3;
// use amethyst_rendy::palette::Srgba;

#[derive(SystemDesc)]
pub struct PlayerCollisionSystem;
const PLAYER_RADIUS_SQUARED: f32 = 24.0 * 24.0;

impl<'s> System<'s> for PlayerCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Planet>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            mut players,
            transforms,
            planets,
            entities, // , mut debug_lines_resource
        ): Self::SystemData,
    ) {
        //There should only be 1 player
        // let orangeish = Srgba::new(1.0, 0.6, 0.0, 1.0);
        for (player, player_entity) in (&mut players, &entities).join() {
            if let Some(player_transform) = transforms.get(player_entity) {
                // let player_translate = player_transform.translation();
                for (planet, planet_transform) in (&planets, &transforms).join() {
                    let distance_sqrt = distance_squared(player_transform, planet_transform);
                    if distance_sqrt <= planet.radius_squared + PLAYER_RADIUS_SQUARED {
                        player.is_dead = true;
                    }
                }
            }
        }
    }
}
