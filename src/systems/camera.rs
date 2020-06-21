use crate::components::Player;
use crate::components::Velocity;
use crate::playercamera::{CAMERA_HEIGHT, CAMERA_WIDTH};

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
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Velocity>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut cameras, mut transforms, players, velocities, entities): Self::SystemData,
    ) {
        // let delta_time = time.delta_seconds();
        for (player_entity, _player, player_velocity) in (&entities, &players, &velocities).join() {
            if let Some(player_transform) = transforms.get(player_entity) {
                let player_position = player_transform.translation().clone();
                for (camera_entity, camera) in (&entities, &mut cameras).join() {
                    if let Some(camera_transform) = transforms.get_mut(camera_entity) {
                        let velocity = player_velocity.velocity;
                        let scale = f32::min(f32::max(1.0, velocity.magnitude() / 100.0), 3.0);
                        let scaled_projection = get_scaled_camera_projection(scale);
                        camera.set_projection(scaled_projection);
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
