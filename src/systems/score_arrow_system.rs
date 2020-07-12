use amethyst::core::math::Vector2;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};
use amethyst_rendy::{ resources::Tint};
use std::f64::consts::PI;

use crate::components::{Player, ScoreArea, ScoreArrow};

#[derive(SystemDesc)]
pub struct ScoreArrowSystem;

impl<'s> System<'s> for ScoreArrowSystem {
    type SystemData = (
        ReadStorage<'s, ScoreArrow>,
        ReadStorage<'s, ScoreArea>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tint>,
    );

    fn run(&mut self, (arrows, score_areas, players, mut transforms, mut tints): Self::SystemData) {
        let score_pos = {
            let mut x: f32 = 0.0;
            let mut y: f32 = 0.0;
            for (_score_area, score_area_transform) in (&score_areas, &transforms).join() {
                let trans = score_area_transform.translation();
                x = trans.x;
                y = trans.y;
            }
            Vector2::new(x, y)
        };

        let player_pos = {
            let mut x = 0.0;
            let mut y = 0.0;
            for (_player, player_transform) in (&players, &transforms).join() {
                let trans = player_transform.translation();
                x = trans.x;
                y = trans.y;
            }
            Vector2::new(x, y)
        };

        let player_to_score: Vector2<f32> = score_pos - player_pos;
        let player_to_score_length = player_to_score.magnitude();
        if player_to_score_length > 0.01 {
            let player_to_score_direction = player_to_score.normalize();
            let arrow_distance = clamp(player_to_score_length, 40.0, 100.0);
            let arrow_position = player_pos + player_to_score_direction * arrow_distance;
            for (_arrow, arrow_transform, tint) in (&arrows, &mut transforms, &mut tints).join() {
                let up = Vector2::new(0.0, 1.0);
                let mut angle = up.angle(&player_to_score_direction);
                if player_to_score_direction.x > 0.0 {
                    angle = 2.0 * PI as f32 - angle;
                }
                arrow_transform.set_translation_xyz(arrow_position.x, arrow_position.y, 0.0);
                arrow_transform.set_rotation_2d(angle);
                if player_to_score_length < 600.0 {
                    tint.0.alpha = 0.0;
                } else {
                    tint.0.alpha = 1.0;
                }
            }
        }
    }
}

pub fn clamp(input: f32, min: f32, max: f32) -> f32 {
    if input < min {
        return min;
    }
    if input > max {
        return max;
    }
    input
}
