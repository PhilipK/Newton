use crate::components::Player;
use crate::components::ScoreArea;
use crate::components::ZoomCamera;
use crate::playercamera::{CAMERA_HEIGHT, CAMERA_WIDTH};
use crate::utils::distance_squared_vec;
use amethyst::core::timing::Time;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
    renderer::{camera::Projection, Camera},
};

#[derive(SystemDesc)]
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        WriteStorage<'s, Camera>,
        WriteStorage<'s, ZoomCamera>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Entities<'s>,
        ReadStorage<'s, ScoreArea>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            mut cameras,
            mut zoom_cameras,
            mut transforms,
            players,
            input,
            entities,
            score_areas,
            time,
        ): Self::SystemData,
    ) {
        let zoom_speed = 0.5;
        let max_zoom_level = 10.0;
        let min_zoom_level = 0.5;

        for (player_entity, _player) in (&entities, &players).join() {
            if let Some(player_transform) = transforms.get(player_entity) {
                let player_position = player_transform.translation().clone();
                for (camera_entity, camera, zoom_camera) in
                    (&entities, &mut cameras, &mut zoom_cameras).join()
                {
                    for (score_entity, _score_area) in (&entities, &score_areas).join() {
                        let score_transform_cl =
                            { transforms.get(score_entity).unwrap().translation().clone() };
                        if let Some(camera_transform) = transforms.get_mut(camera_entity) {
                            if let Some(zoom_axis) = input.axis_value("zoom") {
                                zoom_camera.zoom_level = clamp(
                                    zoom_camera.zoom_level + (zoom_axis * -1.0) * zoom_speed,
                                    min_zoom_level,
                                    max_zoom_level,
                                );
                                let scaled_projection =
                                    get_scaled_camera_projection(zoom_camera.zoom_level);
                                camera.set_projection(scaled_projection);
                            }

                            let mut tx = player_position.x;
                            let mut ty = player_position.y;

                            let distance =
                                distance_squared_vec(&player_position, &score_transform_cl).sqrt();
                            let max_distance = 450.0;
                            let min_distance = 250.0;
                            if distance < max_distance {
                                let mut percent = 0.0;
                                if distance > min_distance {
                                    percent =
                                        (distance - min_distance) / (max_distance - min_distance);
                                }
                                tx = lerp(score_transform_cl.x, player_position.x, percent);
                                ty = lerp(score_transform_cl.y, player_position.y, percent);
                            }
                            let delta_seconds = time.delta_seconds() * 6.0;
                            {
                                let cur_camera_pos = camera_transform.translation();
                                let (cx, cy) = (cur_camera_pos.x, cur_camera_pos.y);
                                camera_transform.set_translation_xyz(
                                    lerp(cx, tx, delta_seconds),
                                    lerp(cy, ty, delta_seconds),
                                    1.0,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn lerp(a: f32, b: f32, percent: f32) -> f32 {
    (1.0 - percent) * a + percent * b
}

fn clamp(input: f32, min: f32, max: f32) -> f32 {
    if input < min {
        return min;
    }
    if input > max {
        return max;
    }
    input
}

fn get_scaled_camera_projection(scale: f32) -> Projection {
    Projection::orthographic(
        -CAMERA_WIDTH / 2.0 * scale,
        CAMERA_WIDTH / 2.0 * scale,
        -CAMERA_HEIGHT / 2.0 * scale,
        CAMERA_HEIGHT / 2.0 * scale,
        0.1,
        2000.0,
    )
}
