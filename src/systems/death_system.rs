use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::ParallelIterator;
use amethyst::ecs::{ParJoin, ReadStorage, System, SystemData, WriteStorage};

use crate::components::Acceleration;
use crate::components::Force;
use crate::components::Mass;

#[derive(SystemDesc)]
pub struct DeathSystem;

impl<'s> System<'s> for DeathSystem {
    type SystemData = (ReadStorage<'s, Player>,);

    fn run(&mut self, (players): Self::SystemData) {
        for (player) in (&players) {
            if(player.is_dead){
                println!("Player is dead");
            }
        }
    }
}
