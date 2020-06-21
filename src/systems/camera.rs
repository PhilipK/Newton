use crate::components::Player;

use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
    renderer::Camera,
};

#[derive(SystemDesc)]
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Entities<'s>,
    );

    fn run(&mut self, (cameras, mut transforms, players, entities): Self::SystemData) {
        // let delta_time = time.delta_seconds();
        for (player_entity, _player) in (&entities, &players).join() {
            if let Some(player_transform) = transforms.get(player_entity) {
                let player_position = player_transform.translation().clone();
                for (camera_entity, _camera) in (&entities, &cameras).join() {
                    if let Some(camera_transform) = transforms.get_mut(camera_entity) {
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
