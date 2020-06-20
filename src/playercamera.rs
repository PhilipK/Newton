
use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera},
};

pub const CAMERA_HEIGHT: f32 = 1000.0;
pub const CAMERA_WIDTH: f32 = 1000.0;

pub fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
        .with(transform)
        .build();
}
