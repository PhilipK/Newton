use crate::components::Player;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct PlayerAnimationSystem;

impl<'s> System<'s> for PlayerAnimationSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (players, mut renderers, input, time): Self::SystemData) {
        let steps_pr_second = 10.0;
        let animation_steps = 4;
        for (_player, renderer) in (&players, &mut renderers).join() {
            if let Some(value) = input.axis_value("throttle") {
                if value > 0.0 {
                    let absolute_seconds = time.absolute_time_seconds();
                    let seconds = (((absolute_seconds * steps_pr_second).round() as u32)
                        % animation_steps
                        + 1) as usize;
                    renderer.sprite_number = seconds;
                } else {
                    renderer.sprite_number = 0;
                }
            }
        }
    }
}
