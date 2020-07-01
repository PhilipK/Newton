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
use amethyst::ecs::{
    Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData, Write, WriteStorage,
};
use amethyst::renderer::SpriteRender;
use amethyst::ui::UiText;
use std::ops::Deref;

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
        Read<'s, LazyUpdate>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        Read<'s, AssetStorage<Source>>,
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
            lazy,
            sprite_resource,
            sounds,
            audio_output,
            storage,
        ): Self::SystemData,
    ) {
        let delta_seconds = time.delta_seconds();
        let mut rng = rand::thread_rng();
        let time_limit = 3.0;
        let sprite_steps = 4;
        let mut scored = false;
        let mut player_position_option: Option<Transform> = None;
        for (_player, player_entity) in (&players, &entities).join() {
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
                                let score_ui_text = ui_texts.get_mut(score_text.score).unwrap();
                                score_ui_text.text = score_board.score.to_string();
                            } else {
                                if time_before.ceil() > score_area.time_left.ceil() {
                                    //we scored
                                    play_score_tick_sound(
                                        &*sounds,
                                        &storage,
                                        audio_output.as_ref().map(|o| o.deref()),
                                    );
                                }
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
        if scored {
            if let Some(player_position) = player_position_option {
                for (_score_area, score_area_transform) in (&score_areas, &mut transforms).join() {
                    let (rnd_x, rnd_y) = (
                        rng.gen::<f32>() * 2000.0 - 1000.0,
                        rng.gen::<f32>() * 2000.0 - 1000.0,
                    );
                    score_area_transform.set_translation_xyz(rnd_x, rnd_y, 0.0);
                    //create score arrow
                    let arrow_builder = lazy.create_entity(&entities);
                    let _entity = spawn_score_arrow(
                        arrow_builder,
                        &player_position,
                        score_area_transform,
                        sprite_resource.arrow_sprite_sheet.clone(),
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
