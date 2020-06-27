use crate::components::Planet;
use crate::components::Player;
use crate::utils::distance_squared;

use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, ReadStorage, System, SystemData};
// use amethyst::ecs::{Write};
// use amethyst::renderer::debug_drawing::DebugLines;
// use amethyst_core::math::Point3;
// use amethyst_rendy::palette::Srgba;

#[derive(SystemDesc)]
pub struct PlayerCollisionSystem;
const PLAYER_RADIUS_SQUARED: f32 = 24.0 * 24.0;

impl<'s> System<'s> for PlayerCollisionSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Planet>,
        Entities<'s>,
        // Write<'s, DebugLines>, // Request DebugLines resource
    );

    fn run(
        &mut self,
        (players, transforms, planets, entities
            // , mut debug_lines_resource
        ): Self::SystemData,
    ) {
        //There should only be 1 player
        // let orangeish = Srgba::new(1.0, 0.6, 0.0, 1.0);
        for (_player, player_entity) in (&players, &entities).join() {
            if let Some(player_transform) = transforms.get(player_entity) {
                // let player_translate = player_transform.translation();
                for (planet, planet_transform) in (&planets, &transforms).join() {
                    // let planet_translate = planet_transform.translation();
                    // let center = Point3::<f32>::new(
                    //     planet_translate.x,
                    //     planet_translate.y,
                    //     planet_translate.z,
                    // );
                    // let player_center = Point3::<f32>::new(
                    //     player_translate.x,
                    //     player_translate.y,
                    //     player_translate.z,
                    // );

                    // debug_lines_resource.draw_circle(center, planet.radius, 36, orangeish);
                    // debug_lines_resource.draw_line(center, player_center, orangeish);
                    let distance_sqrt = distance_squared(player_transform, planet_transform);
                    if distance_sqrt <= planet.radius_squared + PLAYER_RADIUS_SQUARED {
                        println!("Collision");
                    }
                }
            }
        }
    }
}
