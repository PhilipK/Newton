use amethyst::core::{timing::Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};

use crate::components::Acceleration;
use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct AccelerationToVelocitySystem;

impl<'s> System<'s> for AccelerationToVelocitySystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Acceleration>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut velocities, accelerations, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        for (acceleration, velocity) in (&accelerations, &mut velocities).join() {
            velocity.accelerate(
                acceleration.acceleration.x * delta_time,
                acceleration.acceleration.y * delta_time,
            );
        }
    }
}
