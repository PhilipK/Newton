use amethyst::{core::transform::Transform, prelude::*, renderer::Camera};

pub const CAMERA_HEIGHT: f32 = 1000.0;
pub const CAMERA_WIDTH: f32 = 1000.0;


pub fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    let zoom_out_level = 1.0;
    transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(CAMERA_WIDTH * zoom_out_level, CAMERA_HEIGHT * zoom_out_level))
        .with(transform)
        .build();
}
