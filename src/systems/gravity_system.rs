use crate::components::Force;
use crate::components::Gravity;
use crate::components::Mass;
use amethyst::core::{timing::Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::ParallelIterator;

use amethyst::ecs::{Join, ParJoin, Read, ReadStorage, System, SystemData, WriteStorage};

#[derive(SystemDesc)]
pub struct GravitySystem;

const GRAVITATIONAL_CONSTANT: f32 = 1.0;
// const MAX_DISTANCE: f32 = 1000.0 * 1000.0;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        WriteStorage<'s, Force>,
        ReadStorage<'s, Mass>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Gravity>,
        Read<'s, Time>,
    );

    /// Gravity: https://www.khanacademy.org/science/ap-physics-1/ap-centripetal-force-and-gravitation/newtons-law-of-gravitation-ap/a/newtons-law-of-gravitation-ap1
    /// https://www.nuclear-power.net/wp-content/uploads/gravitational-force-equation.png
    /// F = G * (m1*m2)/(r*r)
    fn run(&mut self, (mut forces, masses, transforms, gravities, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        (&mut forces, &masses, &transforms)
            .par_join()
            .for_each(|(force1, mass1, transform1)| {
                let translate1 = transform1.translation();
                for (mass2, transform2, _gravity) in (&masses, &transforms, &gravities).join() {
                    if transform1 != transform2 {
                        let translate2 = transform2.translation();
                        let distance_squared = (translate1.x - translate2.x)
                            * (translate1.x - translate2.x)
                            + (translate1.y - translate2.y) * (translate1.y - translate2.y);
                        if distance_squared > 0.0 {
                            let forceg = GRAVITATIONAL_CONSTANT
                                * (mass1.mass * mass2.mass / distance_squared);
                            let direction = translate2 - translate1;
                            let forceg_timed = forceg * delta_time * direction;
                            force1.add_force(forceg_timed.x, forceg_timed.y);
                        }
                    }
                }
            });
    }
}
