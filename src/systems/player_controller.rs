use crate::resources::{pause_thrust_sound, play_thrust_sound, Sounds};
use amethyst::assets::AssetStorage;
use amethyst::audio::Source;
use amethyst::core::math::Vector3;
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;

use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::Force;
use crate::components::Player;

#[derive(SystemDesc)]
pub struct PlayerControlllerSystem;

impl<'s> System<'s> for PlayerControlllerSystem {
    type SystemData = (
        WriteStorage<'s, Force>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        ReadExpect<'s, Sounds>,
        Read<'s, AssetStorage<Source>>,
    );

    fn run(
        &mut self,
        (mut forces, players, mut tarnsforms, input, time, sounds, storage): Self::SystemData,
    ) {
        for (force, player, transform) in (&mut forces, &players, &mut tarnsforms).join() {
            let delta_time = time.delta_seconds();
            if let Some(rotate_value) = input.axis_value("rotate") {
                if rotate_value != 0.0 {
                    transform.rotate_2d(rotate_value * delta_time * player.turn_pr_second);
                }
            }

            if let Some(value) = input.axis_value("throttle") {
                if value > 0.0 {
                    let power = player.forward_thrust_power;
                    let throttle = value * delta_time * power;
                    let direction = transform.isometry().inverse().rotation * -Vector3::y();
                    let add_force = direction * throttle;
                    force.add_force(add_force.x, add_force.y * -1.0);
                    play_thrust_sound(&sounds, &storage);
                } else {
                    pause_thrust_sound(&*sounds);
                }
            }
        }
    }
}
