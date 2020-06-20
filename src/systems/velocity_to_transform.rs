use amethyst::core::{timing::Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};

use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct VelocityToTransformSystem;

impl<'s> System<'s> for VelocityToTransformSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            let v = velocity.velocity;
            transform.prepend_translation_x(v[0] * delta_time);
            transform.prepend_translation_y(v[1] * delta_time);
        }
    }
}
