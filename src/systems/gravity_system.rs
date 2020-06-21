use amethyst::core::{timing::Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};

use crate::components::Force;
use crate::components::Mass;

#[derive(SystemDesc)]
pub struct GravitySystem;

const GRAVITATIONAL_CONSTANT: f32 = 1.0;
const MAX_DISTANCE: f32 = 1000.0 * 1000.0;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        WriteStorage<'s, Force>,
        ReadStorage<'s, Mass>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
    );

    /// Gravity: https://www.khanacademy.org/science/ap-physics-1/ap-centripetal-force-and-gravitation/newtons-law-of-gravitation-ap/a/newtons-law-of-gravitation-ap1
    /// https://www.nuclear-power.net/wp-content/uploads/gravitational-force-equation.png
    /// F = G * (m1*m2)/(r*r)
    fn run(&mut self, (mut forces, masses, transforms, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        let mut i = 0;
        for (force1, mass1, transform1) in (&mut forces, &masses, &transforms).join() {
            let translate1 = transform1.translation();
            for (mass2, transform2) in (&masses, &transforms).join() {
                if transform1 != transform2 {
                    let translate2 = transform2.translation();
                    let distance_squared = (translate1.x - translate2.x)
                        * (translate1.x - translate2.x)
                        + (translate1.y - translate2.y) * (translate1.y - translate2.y);
                    if  distance_squared > 0.0 {
                        let forceg =
                            GRAVITATIONAL_CONSTANT * (mass1.mass * mass2.mass / distance_squared);
                        let direction = translate2 - translate1;
                        let forceg_timed = forceg * delta_time * direction;
                        force1.add_force(forceg_timed.x, forceg_timed.y);
                        i += 1;
                    }
                }
            }
        }
        println!("{}", i);
    }
}
