use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

use crate::components::Acceleration;
use crate::components::Force;
use crate::components::Mass;

#[derive(SystemDesc)]
pub struct ForceToAcceletationSystem;

impl<'s> System<'s> for ForceToAcceletationSystem {
    type SystemData = (
        WriteStorage<'s, Acceleration>,
        WriteStorage<'s, Force>,
        ReadStorage<'s, Mass>        
    );

    fn run(&mut self, (mut accelerations, mut forces, masses): Self::SystemData) {        
        for (acceleration, force, mass) in (&mut accelerations, &mut forces, &masses).join() {
            let delta_acc = force.force * mass.mass;
            acceleration.add_acceleration(delta_acc.x, delta_acc.y);
            force.force *= 0.0;
        }
    }
}
