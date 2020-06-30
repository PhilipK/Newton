use crate::components::SimpleAnimation;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::ParallelIterator;
use amethyst::ecs::{ParJoin, Read, System, SystemData, WriteStorage};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct SimpleAnimationSystem;

impl<'s> System<'s> for SimpleAnimationSystem {
    type SystemData = (
        WriteStorage<'s, SimpleAnimation>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut animations, mut renders, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        (&mut animations, &mut renders)
            .par_join()
            .for_each(|(animation, renders)| {
                animation.time_left -= delta_time;
                if animation.time_left < 0.0 {
                    //next sprite
                    animation.time_left = animation.time_pr_index + animation.time_left;
                    animation.current_index += 1;
                    if animation.current_index > animation.max_index {
                        animation.current_index = animation.min_index;
                    }
                    renders.sprite_number = animation.current_index;
                }
            });
    }
}
