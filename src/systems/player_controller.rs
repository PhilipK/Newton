use amethyst::core::math::Vector2;
use amethyst::core::math::Vector3;
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::Force;
use crate::components::Player;

#[derive(SystemDesc)]
pub struct PlayerControlllerSystem;

impl<'s> System<'s> for PlayerControlllerSystem {
    type SystemData = (
        WriteStorage<'s, Force>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut forces, players, tarnsforms, input, time): Self::SystemData) {
        for (force, player, transform) in (&mut forces, &players, &tarnsforms).join() {
            // let rotate = input.axis_value("rotate");
            let delta_time = time.delta_seconds();
            let throttle = match input.axis_value("throttle") {
                Some(value) => value * delta_time * player.forward_thrust_power,
                None => 0.0,
            };
            if throttle != 0.0 {
                println!("throttle {}", throttle);
            }

            let player_up_3d = transform.isometry().inverse().rotation * Vector3::y();
            let player_up_2d = Vector2::new(player_up_3d.x, player_up_3d.y);
            let add_force = player_up_2d * throttle;
            force.add_force(add_force.x, add_force.y);
        }
    }
}
