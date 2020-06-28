use crate::components::{Player, ScoreArea};
use amethyst::core::{timing::Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage};
use rand::Rng;

#[derive(SystemDesc)]
pub struct ScoreSystem;

impl<'s> System<'s> for ScoreSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, ScoreArea>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (players, mut transforms, mut score_areas, time, entities): Self::SystemData,
    ) {
        let delta_seconds = time.delta_seconds();
        let mut rng = rand::thread_rng();
        for (_player, player_entity) in (&players, &entities).join() {
            if let Some(player_transform) = transforms.get(player_entity) {
                let player_position = player_transform.clone();
                for (score_area, score_entity) in (&mut score_areas, &entities).join() {
                    if let Some(score_area_transform) = transforms.get_mut(score_entity) {
                        if is_in_box(&player_position, score_area_transform, 64.0, 128.0) {
                            score_area.time_left -= delta_seconds;
                            println!("time: {}", score_area.time_left);
                            if score_area.time_left <= 0.0 {
                                score_area.time_left = 3.0;
                                score_area_transform.set_translation_xyz(
                                    rng.gen::<f32>() * 2000.0 - 1000.0,
                                    rng.gen::<f32>() * 2000.0 - 1000.0,
                                    0.0,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_in_box(player: &Transform, score: &Transform, height: f32, width: f32) -> bool {
    let pt = player.translation();
    let sat = score.translation();
    sat.x - width * 0.5 <= pt.x
        && pt.x <= sat.x + width * 0.5
        && sat.y - height * 0.5 <= pt.y
        && pt.y <= sat.y + height * 0.5
}
