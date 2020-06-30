use crate::components::{Player, ScoreArea};
use crate::entities::score_board::{ScoreBoard, ScoreText};
use amethyst::core::{timing::Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData, Write, WriteStorage,
};
use amethyst::renderer::SpriteRender;
use amethyst::ui::UiText;
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
        Write<'s, ScoreBoard>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ScoreText>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (
            players,
            mut transforms,
            mut score_areas,
            time,
            entities,
            mut score_board,
            mut ui_texts,
            score_text,
            mut sprites,
        ): Self::SystemData,
    ) {
        let delta_seconds = time.delta_seconds();
        let mut rng = rand::thread_rng();
        let time_limit = 3.0;
        let sprite_steps = 4;
        for (_player, player_entity) in (&players, &entities).join() {
            if let Some(player_transform) = transforms.get(player_entity) {
                let player_position = player_transform.clone();
                for (score_area, score_entity, sprite) in
                    (&mut score_areas, &entities, &mut sprites).join()
                {
                    if let Some(score_area_transform) = transforms.get_mut(score_entity) {
                        let is_in = is_in_box(&player_position, score_area_transform, 64.0, 128.0);
                        if is_in {
                            score_area.time_left -= delta_seconds;
                            if score_area.time_left <= 0.0 {
                                score_area.time_left = time_limit;
                                score_area_transform.set_translation_xyz(
                                    rng.gen::<f32>() * 2000.0 - 1000.0,
                                    rng.gen::<f32>() * 2000.0 - 1000.0,
                                    0.0,
                                );
                                score_board.score += 1;
                                let score_ui_text = ui_texts.get_mut(score_text.score).unwrap();
                                score_ui_text.text = score_board.score.to_string();
                            }
                        }
                        if score_area.time_left > 0.0 {
                            let sprite_offset = match is_in {
                                true => sprite_steps,
                                false => 0,
                            };

                            let progress = score_area.time_left / time_limit;
                            let sprite_step = (1.0 - progress) * (sprite_steps as f32);
                            sprite.sprite_number = (sprite_step as usize) + sprite_offset;
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
