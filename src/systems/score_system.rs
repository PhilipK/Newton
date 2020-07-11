use crate::components::{Player, ScoreArea};
use crate::entities::score_board::{ScoreBoard, ScoreText};
use crate::newton::spawn_score_arrow;
use crate::resources::SpriteResource;
use crate::resources::{play_score_sound, play_score_tick_sound, Sounds};
use amethyst::assets::AssetStorage;
use amethyst::audio::{output::Output, Source};
use amethyst::core::{timing::Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::LazyUpdate;
use amethyst::ecs::{Entities, Join, Read, ReadExpect, System, SystemData, Write, WriteStorage};
use amethyst::renderer::SpriteRender;
use amethyst::ui::UiText;
use std::ops::Deref;

use rand::Rng;

use crate::systems::wrap_around_system::{BOX_X_MAX, BOX_Y_MAX};

pub const SCORE_AREA_SAFE_ZONE: f32 = 300.0;

pub const TIME_TO_SCORE: f32 = 1.0;

#[derive(SystemDesc)]
pub struct ScoreSystem;

impl<'s> System<'s> for ScoreSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, ScoreArea>,
        Read<'s, Time>,
        Entities<'s>,
        Write<'s, ScoreBoard>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ScoreText>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, LazyUpdate>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        Read<'s, AssetStorage<Source>>,
    );

    fn run(
        &mut self,
        (
            mut players,
            mut transforms,
            mut score_areas,
            time,
            entities,
            mut score_board,
            mut ui_texts,
            score_text,
            mut sprites,
            lazy,
            sprite_resource,
            sounds,
            audio_output,
            storage,
        ): Self::SystemData,
    ) {
        let delta_seconds = time.delta_seconds();
        let mut rng = rand::thread_rng();
        let time_limit = TIME_TO_SCORE;
        let sprite_steps = 4;
        let mut scored = false;
        let mut player_position_option: Option<Transform> = None;
        for (player, player_entity) in (&mut players, &entities).join() {
            if let Some(player_transform) = transforms.get(player_entity) {
                let player_position = player_transform;
                player_position_option = Some(player_position.clone());
                for (score_area, score_entity, sprite) in
                    (&mut score_areas, &entities, &mut sprites).join()
                {
                    if let Some(score_area_transform) = transforms.get(score_entity) {
                        let is_in = is_in_box(&player_position, score_area_transform, 64.0, 128.0);
                        if is_in {
                            let time_before = score_area.time_left;
                            score_area.time_left -= delta_seconds;
                            if score_area.time_left <= 0.0 {
                                //we scored
                                play_score_sound(
                                    &*sounds,
                                    &storage,
                                    audio_output.as_ref().map(|o| o.deref()),
                                );

                                score_area.time_left = time_limit;
                                scored = true;
                                
                                score_board.score += 1;
                                player.time_remaining = (30.0  - (score_board.score as f32)).max(15.0);
                                let score_ui_text = ui_texts.get_mut(score_text.score).unwrap();
                                score_ui_text.text = score_board.score.to_string();
                            } else {
                                if time_before.ceil() > score_area.time_left.ceil() {
                                    //we are in score area
                                    play_score_tick_sound(
                                        &*sounds,
                                        &storage,
                                        audio_output.as_ref().map(|o| o.deref()),
                                    );
                                }
                            }
                        } else {
                            player.time_remaining -= delta_seconds;
                            if player.time_remaining <= 0.0 {
                                player.time_remaining = 0.0;
                                player.is_dead = true;
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
            let timer_ut_text = ui_texts.get_mut(score_text.timer).unwrap();
            timer_ut_text.text = format!("{:.2}",player.time_remaining);
        }

        if scored {
            if let Some(player_position) = player_position_option {
                for (_score_area, score_area_transform) in (&score_areas, &mut transforms).join() {
                    let (rnd_x, rnd_y) = (
                        rng.gen::<f32>() * (BOX_X_MAX - 2.0 * SCORE_AREA_SAFE_ZONE)
                            + SCORE_AREA_SAFE_ZONE,
                        rng.gen::<f32>() * (BOX_Y_MAX - 2.0 * SCORE_AREA_SAFE_ZONE)
                            + SCORE_AREA_SAFE_ZONE,
                    );
                    score_area_transform.set_translation_xyz(rnd_x, rnd_y, 0.0);
                    //create score arrow
                    let arrow_builder = lazy.create_entity(&entities);
                    let _entity = spawn_score_arrow(
                        arrow_builder,
                        &player_position,
                        score_area_transform,
                        sprite_resource.score_arrow_sheet_handle.clone().unwrap(),
                    );
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
