use crate::utils::distance_squared;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, ReadStorage, System, SystemData};

use crate::components::ScoreArea;
use crate::components::ScoreArrow;

#[derive(SystemDesc)]
pub struct ScoreArrowSystem;

impl<'s> System<'s> for ScoreArrowSystem {
    type SystemData = (
        ReadStorage<'s, ScoreArrow>,
        ReadStorage<'s, ScoreArea>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
    );

    fn run(&mut self, (arrows, score_areas, transforms, entities): Self::SystemData) {
        let score_area_distance = 128.0;
        let score_area_distance_squared = score_area_distance * score_area_distance;
        for (_score_area, score_area_transform) in (&score_areas, &transforms).join() {
            for (_arrow, arrow_transform, arrow_entity) in (&arrows, &transforms, &entities).join()
            {
                if distance_squared(score_area_transform, arrow_transform)
                    < score_area_distance_squared
                {
                    let _ = entities.delete(arrow_entity);
                }
            }
        }
    }
}
