use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use amethyst::core::math::Vector2;


use crate::components::Acceleration;
use crate::components::Velocity;
#[derive(SystemDesc)]
pub struct AccelerationToVelocitySystem;

impl<'s> System<'s> for AccelerationToVelocitySystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Acceleration>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut velocities, mut accelerations, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        for (acceleration, velocity) in (&mut accelerations, &mut velocities).join() {
            let acceleration_time = acceleration.acceleration * delta_time;
            velocity.accelerate(acceleration_time.x, acceleration_time.y);
            acceleration.acceleration = Vector2::new(0.0, 0.0);
        }
    }
}
