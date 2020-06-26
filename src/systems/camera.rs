use crate::components::Player;
use crate::components::ZoomCamera;
use crate::playercamera::{CAMERA_HEIGHT, CAMERA_WIDTH};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::{
    core::timing::Time,
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
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut cameras, mut zoom_cameras, mut transforms, players, input, time, entities): Self::SystemData,
    ) {
        let delta_time = time.delta_seconds();
        let zoom_speed = 2.0;

        for (player_entity, _player) in (&entities, &players).join() {
            if let Some(player_transform) = transforms.get(player_entity) {
                let player_position = player_transform.translation().clone();
                for (camera_entity, camera, zoom_camera) in
                    (&entities, &mut cameras, &mut zoom_cameras).join()
                {
                    if let Some(camera_transform) = transforms.get_mut(camera_entity) {
                        if let Some(zoom_axis) = input.axis_value("zoom") {
                            zoom_camera.zoom_level += zoom_axis * delta_time * zoom_speed;
                            let scaled_projection =
                                get_scaled_camera_projection(zoom_camera.zoom_level);
                            camera.set_projection(scaled_projection);
                        }
                        camera_transform.set_translation_xyz(
                            player_position.x,
                            player_position.y,
                            1.0,
                        );
                    }
                }
            }
        }
    }
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
